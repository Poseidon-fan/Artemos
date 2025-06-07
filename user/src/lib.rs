#![no_std]
#![feature(linkage)]

extern crate alloc;

use alloc::vec::Vec;
use core::ptr::addr_of_mut;

use buddy_system_allocator::LockedHeap;

mod console;
mod panic;
mod syscall;

#[global_allocator]
static HEAP: LockedHeap<16> = LockedHeap::empty();

const USER_HEAP_SIZE: usize = 32768;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock().init(addr_of_mut!(HEAP_SPACE) as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start = unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(core::str::from_utf8(unsafe { core::slice::from_raw_parts(str_start as *const u8, len) }).unwrap());
    }
    exit(main(argc, v.as_slice()));
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

pub fn exit(exit_code: i32) -> ! {
    syscall::sys_exit(exit_code);
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    syscall::sys_read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}

pub fn fork() -> isize {
    syscall::sys_fork()
}

pub fn wait(exit_code: &mut i32) -> isize {
    syscall::sys_waitpid(-1, exit_code as *mut _)
}

pub fn exec(path: &str, args: &[*const u8], envp: &[*const u8]) -> isize {
    syscall::sys_exec(path, args, envp)
}
