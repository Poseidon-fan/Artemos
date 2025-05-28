use alloc::{string::String, sync::Arc, vec::Vec};
use core::{any::Any, mem::size_of};

use super::ftype::{VfsError, VfsFileType, VfsResult};

/// VFS inode trait
pub trait VfsInode: Any + Send + Sync {
    /// Reads data from the inode at the given offset
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> VfsResult<usize>;

    /// Writes data to the inode at the given offset
    fn write_at(&self, offset: usize, buf: &[u8]) -> VfsResult<usize>;

    /// Looks up a name in a directory, returning the child inode
    fn lookup(&self, name: &str) -> VfsResult<Arc<dyn VfsInode>>;

    /// Creates a new file, directory, or symlink in a directory
    fn create(&self, name: &str, file_type: VfsFileType, permissions: u16) -> VfsResult<Arc<dyn VfsInode>>;

    /// Removes a file or directory
    fn remove(&self, name: &str) -> VfsResult<()>;

    /// Gets metadata of the inode
    fn metadata(&self) -> VfsResult<Metadata>;

    /// Sets metadata of the inode
    fn set_metadata(&self, metadata: &Metadata) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    /// Resizes the inode
    fn resize(&self, len: usize) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    /// Creates a hard link
    fn link(&self, name: &str, other: &Arc<dyn VfsInode>) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    /// Deletes a hard link
    fn unlink(&self, name: &str) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    /// Renames or moves an inode
    fn rename(&self, old_name: &str, target: &Arc<dyn VfsInode>, new_name: &str) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    /// Lists directory entries
    fn list(&self) -> VfsResult<Vec<String>> {
        Err(VfsError::NotSupported)
    }

    /// Controls device (for device files)
    fn io_control(&self, cmd: u32, data: usize) -> VfsResult<usize> {
        Err(VfsError::NotSupported)
    }

    /// Gets the associated file system
    fn fs(&self) -> Arc<VfsSuperBlock>;

    /// Supports dynamic type casting
    fn as_any_ref(&self) -> &dyn Any;

    //? rcore also support `fn poll` to support async. I don't know if it is needed.
}

/// Metadata of VFS inode
#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    /// Unique inode identifier
    pub inode_number: u64,
    /// File type
    pub file_type: VfsFileType,
    /// Unix-style permissions (e.g., 0o755)
    pub permissions: u16,
    /// File size in bytes
    pub size: u64,
    /// Creation time (Unix timestamp, seconds)
    pub created_at: u64,
    /// Last modification time (Unix timestamp, seconds)
    pub modified_at: u64,
    /// Last access time (Unix timestamp, seconds)
    pub accessed_at: u64,
    /// Number of hard links
    pub link_count: u16,
    /// File system specific flags
    pub flags: u32,
    /// Device ID for device files
    pub rdev: usize,
}
/// Ensure metadata size is reasonable
const _: () = assert!(size_of::<Metadata>() <= 64, "Metadata size too large");

/// VFS super block
pub struct VfsSuperBlock {
    fs_type: String, /* e.g., "ext4", "fat32"
                      * File system specific data (e.g., ext4_rs::Ext4 in future) */
}

impl dyn VfsInode {
    /// Downcasts the inode to a specific type
    pub fn downcast_ref<T: VfsInode>(&self) -> Option<&T> {
        self.as_any_ref().downcast_ref::<T>()
    }
}
