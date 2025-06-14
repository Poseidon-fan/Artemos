use std::{fs::remove_file, path::Path, sync::Arc};

use ext4::{device::EXT4, ext4_inode::Ext4InodeWrapper};
use vfs::{ftype::VfsFileType, superblock::SuperBlock};

fn setup_image() {
    let image = "ext4_fuse/src/ext4.img";
    let path = Path::new(image);
    let _ = remove_file(path);

    let _ = std::fs::File::create(image);
}

#[test]
fn test_mkdir() {
    setup_image();
    let root_inode = Ext4InodeWrapper::root_inode();
    let ext4_device = SuperBlock::new(Arc::new(root_inode), "ext4".to_string()).unwrap();
    ext4_device.create("test", VfsFileType::Directory, 0).unwrap();
    assert!(ext4_device.lookup_path("test").is_ok());
}
