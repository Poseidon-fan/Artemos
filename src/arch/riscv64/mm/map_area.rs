use alloc::collections::btree_map::BTreeMap;

use bitflags::bitflags;

use super::{
    address::{VirtAddr, VirtPageNum},
    frame::FrameTracker,
};

pub struct MapArea {
    vpn_range: (VirtPageNum, VirtPageNum),
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
}

/// kernel area uses direct mapping
/// user area uses frame mapping
pub enum MapType {
    Direct,
    Framed,
}

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
    pub struct MapPermission: u8 {
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
