#![no_std]
#![feature(linkage)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

use buddy_system_allocator::LockedHeap;
use core::ptr::addr_of_mut;
use syscall::*;

const USER_HEAP_SIZE: usize = 32768; //? 原先是16384，变大了
pub const LINUX_REBOOT_MAGIC1: usize = 0xfee1dead;
pub const LINUX_REBOOT_MAGIC2: usize = 672274793;
pub const LINUX_REBOOT_CMD_RESTART: usize = 0x01234567;
pub const LINUX_REBOOT_CMD_HALT: usize = 0xcdef0123;
pub const LINUX_REBOOT_CMD_POWER_OFF: usize = 0x4321fedc;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    unsafe {
        HEAP.lock()
            .init(addr_of_mut!(HEAP_SPACE) as usize, USER_HEAP_SIZE);
    }
    exit(main());
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code)
}

// yield 是 Rust 的关键字，所以取名为 yield_
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    sys_get_time()
}
pub fn getpid() -> isize {
    sys_getpid()
}

/// 对于子进程返回 0，对于当前进程则返回子进程的 PID
pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

/// 返回值：
///
/// 要么返回进程id，要么返回-1。如果要回收的进程还没执行完毕，则会让出cpu
///
/// 与`waitpid`函数不同的是，`waitpid`函数**可以指定pid**，而`wait`函数只能等待任意的子进程回收
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        // 参数`pid==-1`表示等待任意一个子进程
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

/// 返回值：
///
/// 要么返回进程id，要么返回-1。如果要回收的进程还没执行完毕，则会让出cpu
///
/// 与`wait`函数不同的是，`waitpid`函数**可以指定pid**，而`wait`函数因参数`pid==-1`只能等待任意的子进程回收
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            // 要等待的子进程均未结束，则让出cpu
            -2 => {
                yield_();
            }
            // 要等待的子进程不存在，则返回-1
            // 或者成功回收，则返回进程id
            exit_pid => return exit_pid,
        }
    }
}

pub fn sleep(period_ms: usize) {
    let start = sys_get_time();
    while sys_get_time() < start + period_ms as isize {
        sys_yield();
    }
}

pub fn reboot(magic1: usize, magic2: usize, cmd: usize) {
    if magic1 == LINUX_REBOOT_MAGIC1
        && magic2 == LINUX_REBOOT_MAGIC2
        && (cmd == LINUX_REBOOT_CMD_RESTART || cmd == LINUX_REBOOT_CMD_HALT)
    {
        sys_reboot(cmd);
    } else {
        panic!("Invalid reboot magic numbers or command");
    }
}

pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits)
}


/// 功能：当前进程关闭一个文件。
/// 
/// 参数：fd 表示要关闭的文件的文件描述符。
/// 
/// 返回值：如果成功关闭则返回 0 ，否则返回 -1 。可能的出错原因：传入的文件描述符并不对应一个打开的文件。
pub fn close(fd: usize) -> isize { 
    sys_close(fd)
}
