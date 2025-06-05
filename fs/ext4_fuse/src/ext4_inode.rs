use alloc::sync::Arc;

use spin::Mutex;
use vfs::inode::{Metadata, VfsInode};

use crate::{device::EXT4, ext4_inode};

struct Ext4InodeWrapper {
    /// Ext4 inode's metadata. Include ino. Note that ext4-rs use ino to identify an inode.
    meta: Metadata,
    /// Ext4 instance from Ext4_rs. To support multiple filesystem.
    ext4_instance: Arc<Mutex<ext4_rs::Ext4>>,
}

impl VfsInode for Ext4InodeWrapper {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> vfs::ftype::VfsResult<usize> {
        let ino = self.metadata().unwrap().inode_number as u32; //? I don't know if it's ok to directly 'as u32'
        let read_data = self.ext4_instance.lock().read_at(ino, offset, buf);
        match read_data {
            Ok(size) => Ok(size),
            Err(_) => Err(vfs::ftype::VfsError::IoError),
        }
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> vfs::ftype::VfsResult<usize> {
        let ext4 = self.ext4_instance.lock();
        let ino = self.metadata().unwrap().inode_number as u32;
        match ext4.write_at(ino, offset, buf) {
            Ok(size) => Ok(size),
            Err(_) => Err(vfs::ftype::VfsError::IoError),
        }
    }

    fn lookup(&self, name: &str) -> vfs::ftype::VfsResult<std::sync::Arc<dyn VfsInode>> {
        todo!()
    }

    fn create(
        &self,
        name: &str,
        file_type: vfs::ftype::VfsFileType,
        permissions: u16,
    ) -> vfs::ftype::VfsResult<std::sync::Arc<dyn VfsInode>> {
        todo!()
    }

    fn remove(&self, name: &str) -> vfs::ftype::VfsResult<()> {
        todo!()
    }

    fn metadata(&self) -> vfs::ftype::VfsResult<vfs::inode::Metadata> {
        todo!()
    }

    fn fs(&self) -> std::sync::Arc<vfs::filesystem::FileSystem> {
        todo!()
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        todo!()
    }
}
