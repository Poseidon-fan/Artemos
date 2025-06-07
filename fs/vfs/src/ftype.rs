/// VFS error type
#[derive(Debug)]
pub enum VfsError {
    NotFound,     // File or directory not found
    IoError,      // I/O error
    Invalid,      // Invalid operation, parameter or path
    NotDir,       // Not a directory
    NotFile,      // Not a file
    EntryExist,   // Entry already exists
    NotSupported, // Operation not supported
    DeviceError,  // Device-related error
    SymLoop,      // Too many symbolic links
}

/// File type enumeration, aligned with POSIX
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VfsFileType {
    Regular = 0x8000,   // S_IFREG
    Directory = 0x4000, // S_IFDIR
    Symlink = 0xA000,   // S_IFLNK
    Fifo = 0x1000,      // S_IFIFO
    Socket = 0xC000,    // S_IFSOCK
    BlockDev = 0x6000,  // S_IFBLK
    CharDev = 0x2000,   // S_IFCHR
    Other = 0x0000,     // OTHER
}

/// Result type alias
pub type VfsResult<T> = core::result::Result<T, VfsError>;
