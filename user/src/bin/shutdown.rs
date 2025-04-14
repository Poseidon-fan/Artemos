#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{reboot, LINUX_REBOOT_MAGIC1, LINUX_REBOOT_MAGIC2, LINUX_REBOOT_CMD_HALT};

#[unsafe(no_mangle)]
pub fn main() {
    reboot(
        LINUX_REBOOT_MAGIC1,
        LINUX_REBOOT_MAGIC2,
        LINUX_REBOOT_CMD_HALT,
    );
    panic!("shutdown failed!");
}