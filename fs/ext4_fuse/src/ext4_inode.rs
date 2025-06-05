use alloc::sync::Arc;

use vfs::inode::{Metadata, VfsInode};

use crate::{device::EXT4, ext4_inode};

struct Ext4InodeWrapper {
    /// Ext4 inode's metadata. Include ino. Note that ext4-rs use ino to identify an inode.
    meta: Metadata,
    /// Ext4 instance from Ext4_rs. To support multiple filesystem.
    ext4_instance: Arc<ext4_rs::Ext4>,
}

impl VfsInode for Ext4InodeWrapper {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> vfs::ftype::VfsResult<usize> {
        todo!()
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> vfs::ftype::VfsResult<usize> {
        todo!()
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
