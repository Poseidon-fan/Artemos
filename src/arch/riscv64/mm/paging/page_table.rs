use alloc::{vec, vec::Vec};

use super::pte::{PTEFlags, PageTableEntry};
use crate::arch::mm::{
    address::{PhysPageNum, VirtPageNum},
    frame::{FrameTracker, frame_alloc},
    paging::pte,
};

pub struct PageTable {
    root_ppn: PhysPageNum,
    /// Note that these are all internal pages
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn token(&self) -> usize {
        (8usize << 60) | self.root_ppn.0
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(
            !pte.is_valid(),
            "vpn {:x}, va {:x} is mapped before mapping",
            vpn.0,
            vpn.0 << 12
        );
        *pte = PageTableEntry::new(ppn, flags); // TODO does flags need a mask ?
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {} is invalid before unmapping", vpn.0);
        *pte = PageTableEntry::empty();
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).copied()
    }

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
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
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
