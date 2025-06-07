#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{exec, fork, wait};

#[unsafe(no_mangle)]
fn main() -> i32 {
    if fork() == 0 {
        exec("user_shell\0", &[core::ptr::null::<u8>()], &[core::ptr::null::<u8>()]);
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let _pid = wait(&mut exit_code);
        }
    }
    0
}
