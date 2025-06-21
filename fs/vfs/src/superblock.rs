use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
    vec::Vec,
};

use spin::Mutex;

use super::{
    dentry::VfsDentry,
    ftype::{VfsError, VfsFileType, VfsResult},
    inode::VfsInode,
};
use crate::{file::File, inode::Metadata};

/// File system core
pub struct SuperBlock {
    /// e.g. "ext4"
    name: String,
    /// Root directory entry
    root: Arc<VfsDentry>,
    /// Inode cache (inode_number -> inode)
    inode_cache: Mutex<BTreeMap<u64, Arc<dyn VfsInode>>>,
}

impl SuperBlock {
    /// Creates a new file system instance with the given root inode
    ///
    /// Params:
    ///
    /// - `root_inode`: Root inode
    /// - `name`: Device name. E.g. 'ext4'
    pub fn new(root_inode: Arc<dyn VfsInode>, name: String) -> VfsResult<Self> {
        // Verify root is a directory
        if root_inode.metadata()?.file_type != VfsFileType::Directory {
            // EXT4_RS's BUG! Must change root inode's file type to directory
            root_inode
                .set_metadata(&Metadata {
                    file_type: VfsFileType::Directory,
                    ..root_inode.metadata()?
                })
                .unwrap();
        }
        let root_dentry = Arc::new(VfsDentry::new("/", Some(root_inode), Weak::new()));
        Ok(SuperBlock {
            name,
            root: root_dentry,
            inode_cache: Mutex::new(BTreeMap::new()),
        })
    }

    /// Looks up a path, returning the corresponding dentry
    pub fn lookup_path(&self, path: &str) -> VfsResult<Arc<VfsDentry>> {
        let components: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|x| !x.is_empty())
            .collect();
        let mut current: Arc<VfsDentry> = self.root.clone();
        for component in components {
            current = VfsDentry::lookup(&current, component)?;
        }
        Ok(current)
    }

    /// Opens a file, returning a file handle
    pub fn open(&self, path: &str) -> VfsResult<File> {
        let dentry = self.lookup_path(path)?;
        let inode = dentry.inode.as_ref().ok_or(VfsError::NotFound)?;
        // Cache inode
        let inode_number = inode.metadata()?.inode_number;
        {
            let mut cache = self.inode_cache.lock();
            cache.entry(inode_number).or_insert_with(|| inode.clone());
        }
        Ok(File::new(inode.clone()))
    }

    /// Creates a new file or directory
    pub fn create(&self, path: &str, file_type: VfsFileType, permissions: u16) -> VfsResult<Arc<VfsDentry>> {
        let (parent_path, name) = split_path(path)?;
        let parent = self.lookup_path(&parent_path)?;
        VfsDentry::create(&parent, name, file_type, permissions)
    }

    /// Removes a file or directory
    pub fn remove(&self, path: &str) -> VfsResult<()> {
        let (parent_path, name) = split_path(path)?;
        let parent = self.lookup_path(&parent_path)?;
        VfsDentry::remove(&parent, name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}


/// Splits a path into parent path and name
fn split_path(path: &str) -> VfsResult<(String, &str)> {
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        return Err(VfsError::Invalid);
    }
    let last_slash = path.rfind('/').unwrap_or(0);
    let (parent, name) = path.split_at(last_slash);
    let parent = if parent.is_empty() { "/" } else { parent };
    let name = name.trim_start_matches('/');
    if name.is_empty() {
        return Err(VfsError::Invalid);
    }
    Ok((parent.to_string(), name))
}
