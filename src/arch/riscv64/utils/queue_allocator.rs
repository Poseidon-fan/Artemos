use alloc::vec::Vec;

use spin::Mutex;

pub struct QueueAllocator {
    inner: Mutex<QueueAllocatorInner>,
}

struct QueueAllocatorInner {
    current: usize,
    recycled: Vec<usize>,
}

impl QueueAllocator {
    pub fn new() -> Self {
        QueueAllocator {
            inner: Mutex::new(QueueAllocatorInner {
                current: 0,
                recycled: Vec::new(),
            }),
        }
    }

    pub fn alloc(&self) -> usize {
        let mut inner = self.inner.lock();
        if let Some(id) = inner.recycled.pop() {
            id
        } else {
            inner.current += 1;
            inner.current - 1
        }
    }

    pub fn dealloc(&self, id: usize) {
        let mut inner = self.inner.lock();
        inner.recycled.push(id);
    }
}
