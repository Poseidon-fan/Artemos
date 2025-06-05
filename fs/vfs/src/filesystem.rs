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

/// File system core
pub struct FileSystem {
    /// e.g. "ext4"
    name: String,
    /// Root directory entry
    root: Arc<VfsDentry>,
    /// Inode cache (inode_number -> inode)
    inode_cache: Mutex<BTreeMap<u64, Arc<dyn VfsInode>>>,
}

impl FileSystem {
    /// Creates a new file system instance with the given root inode
    pub fn new(root_inode: Arc<dyn VfsInode>, name: String) -> VfsResult<Self> {
        // Verify root is a directory
        if root_inode.metadata()?.file_type != VfsFileType::Directory {
            return Err(VfsError::NotDir);
        }
        let root_dentry = Arc::new(VfsDentry::new("/", Some(root_inode), Weak::new()));
        Ok(FileSystem {
            name,
            root: root_dentry,
            inode_cache: Mutex::new(BTreeMap::new()),
        })
    }

    /// Looks up a path, returning the corresponding dentry
    pub fn lookup_path(&self, path: &str) -> VfsResult<Arc<VfsDentry>> {
        let components: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let mut current = self.root.clone();
        for component in components {
            if component.is_empty() {
                continue;
            }
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

/// File handle for open files
pub struct File {
    inode: Arc<dyn VfsInode>,
    offset: usize,
}

impl File {
    pub fn new(inode: Arc<dyn VfsInode>) -> Self {
        File { inode, offset: 0 }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> VfsResult<usize> {
        let size = self.inode.read_at(self.offset, buf)?;
        self.offset += size;
        Ok(size)
    }

    pub fn write(&mut self, buf: &[u8]) -> VfsResult<usize> {
        let size = self.inode.write_at(self.offset, buf)?;
        self.offset += size;
        Ok(size)
    }

    pub fn seek(&mut self, offset: usize) -> VfsResult<()> {
        self.offset = offset;
        Ok(())
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
