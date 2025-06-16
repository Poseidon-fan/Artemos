use crate::{arch::process::pcb::ProcessControlBlock, loader::get_app_data_by_name};

mod context;
mod pcb;
mod tcb;
mod thread_user_res;

pub fn add_initproc() {
    let elf_data = get_app_data_by_name("initproc").unwrap();
    pcb::ProcessControlBlock::init_initproc(elf_data);
}
