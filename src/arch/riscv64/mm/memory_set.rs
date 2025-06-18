use alloc::vec::Vec;
use core::arch::asm;

use log::info;
use riscv::register::satp::{self, Satp};
use spin::{Lazy, Mutex};
use xmas_elf::ElfFile;

use super::{address::VirtAddr, map_area::MapArea, paging::page_table::PageTable};
use crate::arch::{
    board::qemu::MEMORY_END,
    config::PAGE_SIZE,
    mm::{
        address::VirtPageNum,
        map_area::{AreaType, MapPermission, MapType},
    },
};

pub static KERNEL_SPACE: Lazy<Mutex<MemorySet>> = Lazy::new(|| Mutex::new(MemorySet::new_kernel()));

pub fn activate_kernel_space() {
    KERNEL_SPACE.lock().activate();
}

pub struct MemorySet {
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
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
            area.copy_data(&mut self.page_table, data, offset);
        }
        self.areas.push(area);
    }

    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        info!("kernel satp: {:#x}", memory_set.page_table.token());
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        info!(".stack [{:#x}, {:#x})", sstack as usize, estack as usize);
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
                AreaType::Physical,
            ),
            None,
            0,
        );

        info!("[kernel] new kernel finished");

        memory_set
    }

    pub fn new_from_kernel() -> Self {
        Self {
            page_table: PageTable::new_from_kernel(),
            areas: Vec::new(),
        }
    }

    pub fn activate(&self) {
        let satp = self.page_table.token();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }

    fn map_elf(&mut self, elf: &ElfFile, offset: VirtAddr) -> (VirtPageNum, VirtAddr) {
        let elf_header = elf.header;
        let ph_count = elf_header.pt2.ph_count();

        let mut max_end_vpn = offset.floor();
        // header va is the start va of the first loadable segment
        let mut header_va = 0;
        let mut has_found_header_va = false;

        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // get segment start va and end va
                let start_va: VirtAddr = (ph.virtual_addr() as usize + offset.0).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize + offset.0).into();
                if !has_found_header_va {
                    header_va = start_va.0;
                    has_found_header_va = true;
                }
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let map_area = MapArea::new(start_va, end_va, MapType::Framed, map_perm, AreaType::Elf);
                let data_offset = start_va.page_offset();
                max_end_vpn = map_area.vpn_range_end();
                self.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                    data_offset,
                );
            }
        }
        (max_end_vpn, header_va.into())
    }

    // Create a new memory set from an elf file
    // return the memory set, the entry point and the user stack base
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new_from_kernel();

        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;

        // check magic number
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf file");

        // load program headers
        let ph_count = elf_header.pt2.ph_count();
        let entry_point = elf_header.pt2.entry_point() as usize;
        let (max_end_vpn, _head_va) = memory_set.map_elf(&elf, VirtAddr(0));


        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_heap_bottom: usize = max_end_va.into();
        user_heap_bottom += PAGE_SIZE;

        (memory_set, entry_point, user_heap_bottom)
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
