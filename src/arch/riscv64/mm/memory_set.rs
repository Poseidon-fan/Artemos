use alloc::vec::Vec;
use core::arch::asm;

use riscv::register::satp::{self, Satp};
use spin::{Lazy, Mutex};

use super::{map_area::MapArea, paging::page_table::PageTable};
use crate::{
    arch::{board::qemu::MEMORY_END, config::PAGE_SIZE},
    println,
};

pub static KERNEL_SPACE: Lazy<Mutex<MemorySet>> = Lazy::new(|| Mutex::new(MemorySet::new_kernel()));

pub fn activate_kernel_space() {
    KERNEL_SPACE.lock().activate();
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        println!("kernel satp: {:#x}", memory_set.page_table.token());
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(".bss [{:#x}, {:#x})", sbss_with_stack as usize, ebss as usize);

        println!("physical memory: [{:#x},{:#x})", ekernel as usize, MEMORY_END);

        memory_set
    }

    pub fn activate(&self) {
        let satp = self.page_table.token();
        unsafe {
            satp::write(Satp::from_bits(satp));
            asm!("sfence.vma");
        }
    }
}

unsafe extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
}
