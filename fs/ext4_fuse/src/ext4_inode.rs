#![allow(const_item_mutation)]

use alloc::sync::Arc;

use vfs::inode::{Metadata, VfsInode};

use crate::device::{EXT4, EXT4_RDEV};
const ROOT_INO: u32 = 2;

pub struct Ext4InodeWrapper {
    ino: u32,
}

/// Convert inode number (u32) to Ext4InodeWrapper
impl From<u32> for Ext4InodeWrapper {
    fn from(ino: u32) -> Self {
        let ext4_inode_ref = EXT4.lock().get_inode_ref(ino);
        let md = Metadata {
            inode_number: ext4_inode_ref.inode_num as u64,
            file_type: trans_inode_type(ext4_inode_ref.inode.file_type().bits()),
            permissions: ext4_inode_ref.inode.file_perm().bits(),
            size: ext4_inode_ref.inode.size(),
            created_at: ext4_inode_ref.inode.ctime() as u64,
            modified_at: ext4_inode_ref.inode.mtime() as u64,
            accessed_at: ext4_inode_ref.inode.atime() as u64,
            link_count: ext4_inode_ref.inode.links_count,
            flags: ext4_inode_ref.inode.flags(),
            rdev: EXT4_RDEV,
        };

        let inode = Ext4InodeWrapper { ino };
        inode.set_metadata(&md).unwrap();
        inode
    }
}

impl Ext4InodeWrapper {
    pub fn root_inode() -> Self {
        Ext4InodeWrapper { ino: ROOT_INO }
    }
}

impl VfsInode for Ext4InodeWrapper {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> vfs::ftype::VfsResult<usize> {
        let ino = self.metadata().unwrap().inode_number as u32; //? I don't know if it's ok to directly 'as u32'
        let read_data = EXT4.lock().read_at(ino, offset, buf);
        let old_md = self.metadata().unwrap();
        // new a metadata, only "accessed_at" is modified
        let md = Metadata {
            accessed_at: current_time(),
            ..old_md
        };
        self.set_metadata(&md).unwrap();
        match read_data {
            Ok(size) => Ok(size),
            Err(_) => Err(vfs::ftype::VfsError::IoError),
        }
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> vfs::ftype::VfsResult<usize> {
        let ino = self.metadata().unwrap().inode_number as u32;
        // modify metadata
        let old_md = self.metadata().unwrap();
        let md = Metadata {
            modified_at: current_time(),
            ..old_md
        };
        self.set_metadata(&md).unwrap();

        match EXT4.lock().write_at(ino, offset, buf) {
            Ok(size) => Ok(size),
            Err(_) => Err(vfs::ftype::VfsError::IoError),
        }
    }

    fn lookup(&self, name: &str) -> vfs::ftype::VfsResult<std::sync::Arc<dyn VfsInode>> {
        // check if is dir
        if self.metadata().unwrap().file_type != vfs::ftype::VfsFileType::Directory {
            return Err(vfs::ftype::VfsError::NotDir);
        }
        // get child entries
        let ino = self.metadata().unwrap().inode_number as u32;
        let child_entries = EXT4.lock().dir_get_entries(ino);
        // find a child entry with the given name
        if let Some(entry) = child_entries.iter().find(|&x| x.get_name() == name) {
            let inode = Ext4InodeWrapper::from(entry.inode);
            Ok(Arc::new(inode))
        } else {
            Err(vfs::ftype::VfsError::NotFound)
        }
    }

    fn create(
        &self,
        path: &str,
        filetype: vfs::ftype::VfsFileType,
        permissions: u16,
    ) -> vfs::ftype::VfsResult<Arc<dyn VfsInode>> {
        let create_result = if filetype == vfs::ftype::VfsFileType::Directory {
            match EXT4.lock().dir_mk(path) {
                Ok(ino) => Ok(ino),
                Err(_) => Err(vfs::ftype::VfsError::IoError),
            }
        } else {
            let name = path.split('/').last().unwrap();
            let parent_path = path
                .split('/')
                .take(path.split('/').count() - 1)
                .collect::<Vec<_>>()
                .join("/");
            let parent_path = if parent_path.is_empty() {
                "/".to_string()
            } else {
                "/".to_string() + &parent_path
            };
            let parent_ino = EXT4
                .lock()
                .generic_open(&parent_path, &mut ROOT_INO, false, 0, &mut 0)
                .unwrap();

            match EXT4.lock().create(parent_ino, name, reverse_filetype(filetype)) {
                Ok(inode_ref) => Ok(inode_ref.inode_num as usize),
                Err(_) => Err(vfs::ftype::VfsError::IoError),
            }
        };
        if let Err(e) = create_result {
            return Err(e);
        }
        if create_result.is_err() {
            return Err(vfs::ftype::VfsError::IoError);
        }
        let ino = create_result.unwrap();
        let inode = Ext4InodeWrapper::from(ino as u32);
        let md = Metadata {
            inode_number: inode.ino as u64,
            file_type: filetype,
            permissions,
            size: 0,
            created_at: current_time(),
            modified_at: current_time(),
            accessed_at: 0,
            link_count: 0,
            flags: 0,
            rdev: EXT4_RDEV,
        };
        inode.set_metadata(&md).unwrap();
        Ok(Arc::new(inode))
    }

    fn remove(&self, path: &str) -> vfs::ftype::VfsResult<()> {
        // check file type
        if self.metadata().unwrap().file_type == vfs::ftype::VfsFileType::Directory {
            match EXT4.lock().dir_remove(ROOT_INO, path) {
                Ok(_) => Ok(()),
                Err(_) => Err(vfs::ftype::VfsError::IoError),
            }
        } else {
            match EXT4.lock().file_remove(path) {
                Ok(_) => Ok(()),
                Err(_) => Err(vfs::ftype::VfsError::IoError),
            }
        }
    }

    fn metadata(&self) -> vfs::ftype::VfsResult<vfs::inode::Metadata> {
        let ino_ref = EXT4.lock().get_inode_ref(self.ino);
        let md = Metadata {
            inode_number: ino_ref.inode_num as u64,
            file_type: trans_inode_type(ino_ref.inode.file_type().bits()),
            permissions: ino_ref.inode.file_perm().bits(),
            size: ino_ref.inode.size(),
            created_at: ino_ref.inode.ctime() as u64,
            modified_at: ino_ref.inode.mtime() as u64,
            accessed_at: ino_ref.inode.atime() as u64,
            link_count: ino_ref.inode.links_count,
            flags: ino_ref.inode.flags(),
            rdev: 0,
        };
        Ok(md)
    }

    fn set_metadata(&self, md: &Metadata) -> vfs::ftype::VfsResult<()> {
        // Note: MUST set ext4_rs's metadata simultaneously!
        let ino = self.metadata().unwrap().inode_number as u32;
        EXT4.lock().fuse_setattr(
            ino as u64,
            Some(reverse_filetype(md.file_type) as u32 | md.permissions as u32),
            None,
            None,
            Some(md.size),
            Some(md.accessed_at as u32),
            Some(md.modified_at as u32),
            None, //? what's the diff between 'last change' and 'last modified'
            None,
            Some(md.created_at as u32),
            None,
            None,
            Some(md.flags),
        );
        Ok(())
    }
}

/// Translate ext4 file type to vfs file type
///
/// Params:
/// * `det` - Dir entry type. Ref: ext4_rs/ext4_defs/direntry.r
fn trans_filetype(det: u8) -> vfs::ftype::VfsFileType {
    match det {
        1 => vfs::ftype::VfsFileType::Regular,
        2 => vfs::ftype::VfsFileType::Directory,
        3 => vfs::ftype::VfsFileType::CharDev,
        4 => vfs::ftype::VfsFileType::BlockDev,
        5 => vfs::ftype::VfsFileType::Fifo,
        6 => vfs::ftype::VfsFileType::Socket,
        7 => vfs::ftype::VfsFileType::Symlink,
        _ => vfs::ftype::VfsFileType::Other,
    }
}

fn reverse_filetype(ftype: vfs::ftype::VfsFileType) -> u16 {
    match ftype {
        vfs::ftype::VfsFileType::Regular => 0x8000,
        vfs::ftype::VfsFileType::Directory => 0x4000,
        vfs::ftype::VfsFileType::CharDev => 0x2000,
        vfs::ftype::VfsFileType::BlockDev => 0x6000,
        vfs::ftype::VfsFileType::Fifo => 0x1000,
        vfs::ftype::VfsFileType::Socket => 0x8000 + 0x4000, // 0xC000
        vfs::ftype::VfsFileType::Symlink => 0xA000,
        _ => 0,
    }
}

fn trans_inode_type(itype: u16) -> vfs::ftype::VfsFileType {
    match itype {
        0x8000 => vfs::ftype::VfsFileType::Regular,
        0x4000 => vfs::ftype::VfsFileType::Directory,
        0xA000 => vfs::ftype::VfsFileType::Symlink,
        0x1000 => vfs::ftype::VfsFileType::Fifo,
        0xC000 => vfs::ftype::VfsFileType::Socket,
        0x6000 => vfs::ftype::VfsFileType::BlockDev,
        0x2000 => vfs::ftype::VfsFileType::CharDev,
        _ => vfs::ftype::VfsFileType::Other,
    }
}

/// Get unix timestamp
///
/// Return:
/// * `timestamp` - Unix timestamp
fn current_time() -> u64 {
    let now = std::time::SystemTime::now();
    let timestamp = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    timestamp.as_secs()
}
