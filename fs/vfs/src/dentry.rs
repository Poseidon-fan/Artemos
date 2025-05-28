use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
    vec::Vec,
};

use spin::Mutex;

use super::{
    ftype::{VfsError, VfsFileType, VfsResult},
    inode::VfsInode,
};

/// VFS directory entry
pub struct VfsDentry {
    /// File or directory name
    pub name: String,
    /// Associated inode (None for negative dentries)
    pub inode: Option<Arc<dyn VfsInode>>,
    /// Parent dentry (weak reference to avoid cycles)
    pub parent: Weak<VfsDentry>,
    /// Child dentries (thread-safe cache)
    pub children: Mutex<BTreeMap<String, Arc<VfsDentry>>>,
}

impl Clone for VfsDentry {
    fn clone(&self) -> Self {
        VfsDentry {
            name: self.name.clone(),
            inode: self.inode.clone(),
            parent: self.parent.clone(),
            children: Mutex::new(self.children.lock().clone()),
        }
    }
}

impl VfsDentry {
    /// Creates a new VfsDentry
    pub fn new(name: &str, inode: Option<Arc<dyn VfsInode>>, parent: Weak<VfsDentry>) -> Self {
        VfsDentry {
            name: name.to_string(),
            inode,
            parent,
            children: Mutex::new(BTreeMap::new()),
        }
    }

    /// Looks up a child dentry by name
    pub fn lookup(this: &Arc<VfsDentry>, name: &str) -> VfsResult<Arc<VfsDentry>> {
        // Check cache first
        {
            let children = this.children.lock();
            if let Some(dentry) = children.get(name) {
                return Ok(dentry.clone());
            }
        }

        // Validate directory
        let inode = this.inode.as_ref().ok_or(VfsError::NotFound)?;
        if inode.metadata()?.file_type != VfsFileType::Directory {
            return Err(VfsError::NotDir);
        }

        // Lookup inode and create new dentry
        let child_inode = inode.lookup(name)?;
        let child_dentry = Arc::new(VfsDentry {
            name: name.to_string(),
            inode: Some(child_inode),
            parent: Arc::downgrade(this),
            children: Mutex::new(BTreeMap::new()),
        });

        // Update cache
        this.children.lock().insert(name.to_string(), child_dentry.clone());
        Ok(child_dentry)
    }

    /// Creates a new file or directory
    pub fn create(
        this: &Arc<VfsDentry>,
        name: &str,
        file_type: VfsFileType,
        permissions: u16,
    ) -> VfsResult<Arc<VfsDentry>> {
        // Validate directory
        let inode = this.inode.as_ref().ok_or(VfsError::NotFound)?;
        if inode.metadata()?.file_type != VfsFileType::Directory {
            return Err(VfsError::NotDir);
        }

        // Check if entry exists
        {
            let children = this.children.lock();
            if children.contains_key(name) {
                return Err(VfsError::EntryExist);
            }
        }

        // Create inode and dentry
        let child_inode = inode.create(name, file_type, permissions)?;
        let child_dentry = Arc::new(VfsDentry {
            name: name.to_string(),
            inode: Some(child_inode),
            parent: Arc::downgrade(this),
            children: Mutex::new(BTreeMap::new()),
        });

        // Update cache
        this.children.lock().insert(name.to_string(), child_dentry.clone());
        Ok(child_dentry)
    }

    /// Removes a file or directory
    pub fn remove(this: &Arc<VfsDentry>, name: &str) -> VfsResult<()> {
        // Validate directory
        let inode = this.inode.as_ref().ok_or(VfsError::NotFound)?;
        if inode.metadata()?.file_type != VfsFileType::Directory {
            return Err(VfsError::NotDir);
        }

        // Check if entry exists
        {
            let children = this.children.lock();
            if !children.contains_key(name) {
                return Err(VfsError::NotFound);
            }
        }

        // Remove inode and update cache
        inode.remove(name)?;
        this.children.lock().remove(name);
        Ok(())
    }

    /// Lists all child dentries
    pub fn list(this: &Arc<VfsDentry>) -> VfsResult<Vec<String>> {
        let inode = this.inode.as_ref().ok_or(VfsError::NotFound)?;
        if inode.metadata()?.file_type != VfsFileType::Directory {
            return Err(VfsError::NotDir);
        }
        inode.list()
    }
}
