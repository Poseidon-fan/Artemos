use alloc::vec::Vec;
use core::arch::asm;

use log::info;
use riscv::register::satp::{self, Satp};
use spin::{Lazy, Mutex};

use super::{map_area::MapArea, paging::page_table::PageTable};
use crate::arch::{
    board::qemu::MEMORY_END,
    mm::map_area::{AreaType, MapPermission, MapType},
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

    pub fn push(&mut self, mut area: MapArea, data: Option<&[u8]>, offset: usize) {
        area.map(&mut self.page_table);
        if let Some(data) = data {
            area.copy_data_with_offset(data, offset);
        }
        self.areas.push(area);
    }

    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        info!("kernel satp: {:#x}", memory_set.page_table.token());
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        info!(".bss [{:#x}, {:#x})", sbss_with_stack as usize, ebss as usize);

        info!("physical memory: [{:#x},{:#x})", ekernel as usize, MEMORY_END);

        info!("[kernel]mapping .text section");
        memory_set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Direct,
                MapPermission::R | MapPermission::X,
                AreaType::Elf,
            ),
            None,
            0,
        );

        info!("[kernel]mapping .rodata section");
        memory_set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Direct,
                MapPermission::R,
                AreaType::Elf,
            ),
            None,
            0,
        );

        info!("[kernel]mapping.data section");
        memory_set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Direct,
                MapPermission::R | MapPermission::W,
                AreaType::Elf,
            ),
            None,
            0,
        );

        info!("[kernel]mapping .stack section");
        memory_set.push(
            MapArea::new(
                (sstack as usize).into(),
                (estack as usize).into(),
                MapType::Direct,
                MapPermission::R | MapPermission::W,
                AreaType::Elf,
            ),
            None,
            0,
        );

        info!("[kernel]mapping.bss section");
        memory_set.push(
            MapArea::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Direct,
                MapPermission::R | MapPermission::W,
                AreaType::Elf,
            ),
            None,
            0,
        );

        info!("[kernel]mapping physical memory");
        memory_set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Direct,
                MapPermission::R | MapPermission::W,
                AreaType::Elf,
            ),
            None,
            0,
        );

        info!("[kernel] new kernel finished");

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
    fn sbss();
    fn ebss();
    fn ekernel();
    fn sstack();
    fn estack();
}
