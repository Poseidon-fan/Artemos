#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exec, fork, wait, yield_};

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("hello init");
    println!("hello moca");
    if fork() == 0 {
        exec("user_shell\0"); // 子进程，执行user_shell
    } else {
        // 当前进程
        loop {
            // 循环等待子进程回收（即变成僵尸进程）并回收资源
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
    }
    0
}
