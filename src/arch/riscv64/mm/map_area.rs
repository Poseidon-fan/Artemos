use alloc::collections::btree_map::BTreeMap;
use core::clone;

use bitflags::bitflags;

use super::{
    address::{PhysPageNum, VirtAddr, VirtPageNum},
    frame::{FrameTracker, frame_alloc},
    paging::{page_table::PageTable, pte::PTEFlags},
};
use crate::arch::config::{KERNEL_ADDR_OFFSET, PAGE_SIZE};

pub struct MapArea {
    pub vpn_range: (VirtPageNum, VirtPageNum),
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_perm: MapPermission,
    map_type: MapType,
    area_type: AreaType,
}

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
        area_type: AreaType,
    ) -> Self {
        let start_vpn = start_va.floor();
        let end_vpn = end_va.ceil();
        Self {
            vpn_range: (start_vpn, end_vpn),
            data_frames: BTreeMap::new(),
            map_perm,
            map_type,
            area_type,
        }
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        let (start_vpn, end_vpn) = self.vpn_range;
        (start_vpn.0..end_vpn.0).for_each(|vpn| {
            self.map_one(VirtPageNum(vpn), page_table);
        });
    }

    fn map_one(&mut self, vpn: VirtPageNum, page_table: &mut PageTable) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Direct => {
                ppn = PhysPageNum(vpn.0 - KERNEL_ADDR_OFFSET);
            },
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            },
        }
        let flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();
        page_table.map(vpn, ppn, flags);
    }

    /// Data: at the `offset` of the start va.
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8], offset: usize) {
        assert_eq!(self.map_type, MapType::Framed);

        let mut start: usize = 0;
        let mut page_offset: usize = offset;
        let mut cur_vpn = self.vpn_range_begin();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE - page_offset)];
            let dst = &mut page_table.translate(cur_vpn).unwrap().ppn().bytes_array()[offset..offset + src.len()];
            dst.fill(0);
            dst.copy_from_slice(src);
            start += PAGE_SIZE - offset;
            page_offset = 0;
            if start >= len {
                break;
            }
            cur_vpn.0 += 1;
        }
    }

    pub fn from_existed_map_area(another: &MapArea) -> Self {
        Self {
            vpn_range: another.vpn_range,
            data_frames: BTreeMap::new(),
            map_perm: another.map_perm,
            map_type: another.map_type,
            area_type: another.area_type,
        }
    }

    pub fn vpn_range_begin(&self) -> VirtPageNum {
        self.vpn_range.0
    }

    pub fn vpn_range_end(&self) -> VirtPageNum {
        self.vpn_range.1
    }
}

/// kernel area uses direct mapping
/// user area uses frame mapping
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum MapType {
    Direct,
    Framed,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AreaType {
    /// Segments from elf file, e.g. text, rodata, data, bss
    Elf,
    /// Stack
    Stack,
    /// Brk
    Brk,
    /// Mmap
    Mmap,
    /// For Trap Context
    Trap,
    /// Shared memory
    Shm,
    /// Physical frames(for kernel)
    Physical,
    /// MMIO(for kernel)
    Mmio,
}

bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    #[derive(Clone, Copy)]
    pub struct MapPermission: u16 {
        ///Readable
        const R = 1 << 1;
        ///Writable
        const W = 1 << 2;
        ///Excutable
        const X = 1 << 3;
        ///Accessible in U mode
        const U = 1 << 4;
    }
}
