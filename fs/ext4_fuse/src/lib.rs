#![no_std]
use vfs::device::BlockDriver;
pub struct Ext4FileSystem {}
impl BlockDriver for Ext4FileSystem {
    fn read_at(&self, offset: usize, buf: &mut [u8]) {
        todo!()
    }

    fn write_at(&self, offset: usize, buf: &[u8]) {
        todo!()
    }
}
