//! App management syscalls
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::{get_time_ms};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    // 这里进行了修改，因为内核现在不能自己继续加载下一个程序了
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// 获取当前时间
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}