extern crate alloc;

use core::mem::size_of;

// Maximum number of direct data blocks per inode
const MAX_BLOCKS: usize = 12;

// File type enumeration, stored as u8 for compactness
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Regular = 1,   // Regular file
    Directory = 2, // Directory
    Symlink = 3,   // Symbolic link
}

// Inode structure for a virtual file system
#[derive(Debug, Clone, Copy)]
#[repr(C)] // Ensure C-compatible layout for potential FFI
pub struct Inode {
    // Unique inode identifier
    pub inode_number: u64,
    // File size in bytes
    pub size: u64,
    // Creation time (Unix timestamp, seconds)
    pub created_at: u64,
    // Last modification time (Unix timestamp, seconds)
    pub modified_at: u64,
    // Last access time (Unix timestamp, seconds)
    pub accessed_at: u64,
    // Direct data block indices
    pub data_blocks: [u64; MAX_BLOCKS], // 136byte
    // User ID of the owner
    pub uid: u32,
    // Group ID
    pub gid: u32,
    // Number of hard links
    pub link_count: u16,
    // File permissions (Unix-style, e.g., 0o755)
    pub permissions: u16,
    // File type
    pub file_type: FileType,
}

impl Inode {
    /// Creates a new inode with the given parameters
    ///
    /// # Arguments
    /// * `inode_number` - Unique inode identifier
    /// * `file_type` - Type of the file (Regular, Directory, Symlink)
    /// * `permissions` - Unix-style permissions (e.g., 0o755)
    /// * `uid` - User ID of the owner
    /// * `gid` - Group ID
    /// * `timestamp` - Current time as Unix timestamp (seconds)
    ///
    /// # Returns
    /// A new `Inode` instance
    #[inline]
    pub fn new(inode_number: u64, file_type: FileType, permissions: u16, uid: u32, gid: u32, timestamp: u64) -> Self {
        Inode {
            inode_number,
            file_type,
            permissions,
            uid,
            gid,
            size: 0,
            created_at: timestamp,
            modified_at: timestamp,
            accessed_at: timestamp,
            link_count: 1,
            data_blocks: [0; MAX_BLOCKS],
        }
    }

    /// Updates the access timestamp
    #[inline]
    pub fn update_accessed(&mut self, timestamp: u64) {
        self.accessed_at = timestamp;
    }

    /// Updates the modification timestamp and size
    #[inline]
    pub fn update_modified(&mut self, timestamp: u64, new_size: u64) {
        self.modified_at = timestamp;
        self.size = new_size;
    }

    /// Adds a data block index if space is available
    ///
    /// # Arguments
    /// * `block_id` - Index of the data block
    ///
    /// # Returns
    /// * `Ok(())` if the block was added
    /// * `Err(())` if no space is available
    #[inline]
    pub fn add_data_block(&mut self, block_id: u64) -> Result<(), ()> {
        for slot in self.data_blocks.iter_mut() {
            if *slot == 0 {
                *slot = block_id;
                return Ok(());
            }
        }
        Err(())
    }

    /// Checks if the inode is valid (basic sanity check)
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.inode_number != 0 && self.link_count > 0
    }
}

// Ensure the struct size is reasonable and aligned
// const _: () = assert!(size_of::<Inode>() <= 128, "Inode size too large");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inode_creation() {
        let inode = Inode::new(1, FileType::Regular, 0o644, 1000, 1000, 1234567890);
        assert_eq!(inode.inode_number, 1);
        assert_eq!(inode.file_type, FileType::Regular);
        assert_eq!(inode.permissions, 0o644);
        assert_eq!(inode.uid, 1000);
        assert_eq!(inode.gid, 1000);
        assert_eq!(inode.size, 0);
        assert_eq!(inode.link_count, 1);
        assert_eq!(inode.data_blocks, [0; MAX_BLOCKS]);
        assert!(inode.is_valid());
    }

    #[test]
    fn test_add_data_block() {
        let mut inode = Inode::new(2, FileType::Directory, 0o755, 1000, 1000, 1234567890);
        assert!(inode.add_data_block(42).is_ok());
        assert_eq!(inode.data_blocks[0], 42);
        assert!(inode.add_data_block(43).is_ok());
        assert_eq!(inode.data_blocks[1], 43);
    }
}
