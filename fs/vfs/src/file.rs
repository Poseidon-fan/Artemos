use alloc::sync::Arc;

use crate::{ftype::VfsResult, inode::VfsInode};

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
