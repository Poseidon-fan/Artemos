use std::{fs::remove_file, path::Path, sync::Arc};

use ext4::{device::EXT4, ext4_inode::Ext4InodeWrapper};
use vfs::{ftype::VfsFileType, superblock::SuperBlock};

#[test]
fn test_mkdir() {
    let root_inode = Ext4InodeWrapper::root_inode();
    let ext4_device = SuperBlock::new(Arc::new(root_inode), "ext4".to_string()).unwrap();
    ext4_device.create("/test", VfsFileType::Directory, 0).unwrap();
    assert!(ext4_device.lookup_path("test").is_ok());
}
