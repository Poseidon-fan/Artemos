extern crate alloc;
mod manager;
use core::mem::MaybeUninit;


/// Given a range and iterate sub-range for each block
pub struct BlockIter {
    pub begin: usize,
    pub end: usize,
    pub block_size_log2: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockRange {
    pub block: usize,
    pub begin: usize,
    pub end: usize,
    pub block_size_log2: u8,
}

impl BlockRange {
    pub fn is_empty(&self) -> bool {
        self.end == self.begin
    }

    pub fn len(&self) -> usize {
        self.end - self.begin
    }

    pub fn is_full(&self) -> bool {
        self.len() == (1usize << self.block_size_log2)
    }

    pub fn origin_begin(&self) -> usize {
        (self.block << self.block_size_log2) + self.begin
    }

    pub fn origin_end(&self) -> usize {
        (self.block << self.block_size_log2) + self.end
    }
}

impl Iterator for BlockIter {
    type Item = BlockRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin >= self.end {
            return None;
        }
        let block_size_log2 = self.block_size_log2;
        let block_size = 1usize << block_size_log2;
        let block = self.begin / block_size;
        let begin = self.begin % block_size;
        let end = if block == self.end / block_size {
            self.end % block_size
        } else {
            block_size
        };
        self.begin += end - begin;
        Some(BlockRange {
            block,
            begin,
            end,
            block_size_log2,
        })
    }
}

/// Declares a block of uninitialized memory.
///
/// # Safety
///
/// Never read from uninitialized memory!
#[inline(always)]
pub unsafe fn uninit_memory<T>() -> T {
    unsafe {
        #[allow(clippy::uninit_assumed_init)]
        MaybeUninit::uninit().assume_init()
    }
}

/// Trait for block device operations
pub trait BlockDriver: Send + Sync {
    /// Read data at the given offset (bytes)
    fn read_at(&self, offset: usize, buf: &mut [u8]);
    /// Write data at the given offset (bytes)
    fn write_at(&self, offset: usize, buf: &[u8]);
}
