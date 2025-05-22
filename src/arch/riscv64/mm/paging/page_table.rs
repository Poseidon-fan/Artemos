use alloc::{vec, vec::Vec};

use super::pte::{PTEFlags, PageTableEntry};
use crate::arch::mm::{
    address::{PhysPageNum, VirtPageNum},
    frame::{FrameTracker, frame_alloc},
};

pub struct PageTable {
    root_ppn: PhysPageNum,
    /// Note that these are all internal pages
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().expect("no more frames to alloc");
        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn token(&self) -> usize {
        (8usize << 60) | self.root_ppn.0
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {}

    pub fn unmap(&mut self, vpn: VirtPageNum) {}

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
}
