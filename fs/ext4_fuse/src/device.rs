use std::sync::Arc;

use lazy_static::lazy_static;
use spin::Mutex;

/// Temporarily std fs.
#[derive(Debug)]
pub struct Ext4Device {}
pub const EXT4_RDEV: usize = 0;
impl ext4_rs::BlockDevice for Ext4Device {
    fn read_offset(&self, offset: usize) -> Vec<u8> {
        use std::{
            fs::OpenOptions,
            io::{Read, Seek},
        };
        let mut file = OpenOptions::new().read(true).write(true).open("ex4.img").unwrap();
        let mut buf = vec![0u8; ext4_rs::BLOCK_SIZE as usize];
        let _r = file.seek(std::io::SeekFrom::Start(offset as u64));
        let _r = file.read_exact(&mut buf);

        buf
    }

    fn write_offset(&self, offset: usize, data: &[u8]) {
        use std::{
            fs::OpenOptions,
            io::{Seek, Write},
        };
        let mut file = OpenOptions::new().read(true).write(true).open("ex4.img").unwrap();

        let _r = file.seek(std::io::SeekFrom::Start(offset as u64));
        let _r = file.write_all(&data);
    }
}


lazy_static! {
    pub static ref EXT4: Arc<Mutex<ext4_rs::Ext4>> = Arc::new(Mutex::new(ext4_rs::Ext4::open(Arc::new(Ext4Device {}))));
}
