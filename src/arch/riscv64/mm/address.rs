use super::paging::pte::PageTableEntry;
use crate::arch::config::{KERNEL_ADDR_OFFSET, PAGE_SIZE, PAGE_SIZE_BITS};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

/// convert kernel virtual address to physical address
pub fn kva2pa(vaddr: VirtAddr) -> PhysAddr {
    PhysAddr(vaddr.0 - KERNEL_ADDR_OFFSET)
}

/// convert physical address to kernel virtual address
pub fn pa2kva(paddr: PhysAddr) -> VirtAddr {
    VirtAddr(paddr.0 + KERNEL_ADDR_OFFSET)
}

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
const VA_WIDTH_SV39: usize = 39;


impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}

impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        v.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        // Self(v & ((1 << VA_WIDTH_SV39) - 1))
        let tmp = v as isize >> VA_WIDTH_SV39;
        if tmp != 0 && tmp != -1 {
            log::error!("v {:#x}, tmp {:#x}", v, tmp);
        }
        assert!(tmp == 0 || tmp == -1, "invalid va: {:#x}", v);
        Self(v)
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl PhysPageNum {
    pub fn bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        let kernel_va = pa2kva(pa).0;
        unsafe { core::slice::from_raw_parts_mut(kernel_va as *mut u8, 4096) }
    }

    pub fn pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        let kernel_va = pa2kva(pa).0;
        unsafe { core::slice::from_raw_parts_mut(kernel_va as *mut PageTableEntry, 512) }
    }
}

impl VirtPageNum {
    /// Return VPN 3 level index
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}
