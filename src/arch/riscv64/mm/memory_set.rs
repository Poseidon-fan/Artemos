use alloc::vec::Vec;
use core::arch::asm;

use log::info;
use riscv::register::satp::{self, Satp};
use spin::{Lazy, Mutex};

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

    pub fn activate(&self) {
        let satp = self.page_table.token();
        unsafe {
            satp::write(Satp::from_bits(satp));
            asm!("sfence.vma");
        }
    }

    // Create a new memory set from an elf file
    // return the memory set, the entry point and the user stack base
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new_bare();

        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;

        // check magic number
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf file");

        // load program headers
        let ph_count = elf_header.pt2.ph_count();
        let entry_point = elf_header.pt2.entry_point() as usize;
        let mut max_end_vpn = VirtPageNum(0);

        for i in 0..ph_count {
            // get program header
            let ph = elf.program_header(i).unwrap();
            // if this program header is need to be loaded
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();

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

                let map_area = MapArea::new(start_va.into(), end_va.into(), MapType::Framed, map_perm, AreaType::Elf);
                max_end_vpn = map_area.vpn_range.1;
                let map_offset = start_va.0 - start_va.floor().0 * PAGE_SIZE;
                memory_set.push(
                    map_area,
                    Some(&elf_data[ph.offset() as usize..ph.offset() as usize + ph.file_size() as usize]),
                    map_offset,
                );
            }
        }

        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_base: usize = max_end_va.into();
        user_stack_base += PAGE_SIZE;

        (memory_set, entry_point, user_stack_base)
    }

    // todo: 地址空间嗯映射就可以了吗
    pub fn from_existed_user_space(user_space: &Self) -> Self {
        let mut memory_set = Self::new_bare();
        for area in user_space.areas.iter() {
            let new_area = MapArea::from_existed_map_area(area);
            let vpn_start = new_area.vpn_range.0.0;
            let vpn_end = new_area.vpn_range.1.0;
            // todo: how to set offset
            memory_set.push(new_area, None, 0);

            for vpn in vpn_start..vpn_end {
                let vpn = VirtPageNum(vpn);
                let src_ppn = user_space.page_table.translate(vpn).unwrap().ppn();
                let dst_ppn = memory_set.page_table.translate(vpn).unwrap().ppn();
                dst_ppn.bytes_array().copy_from_slice(src_ppn.bytes_array());
            }
        }
        memory_set
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
