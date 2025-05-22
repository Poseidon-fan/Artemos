use bitflags::bitflags;
use log::info;

use crate::arch::mm::address::PhysPageNum;


bitflags! {
    pub struct PTEFlags: u16 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
        const COW = 1 << 8;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: (ppn.0 << 10) | flags.bits() as usize,
        }
    }

    pub fn ppn(&self) -> PhysPageNum {
        ((self.bits >> 10) & ((1usize << 44) - 1)).into()
    }

    fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits((self.bits & ((1 << 9) - 1)) as u16).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        !(self.flags() & PTEFlags::V).is_empty()
    }
}
