use alloc::vec::Vec;

pub struct QueueAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl QueueAllocator {
    pub fn new() -> Self {
        QueueAllocator {
            current: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }

    // RAII: dealloc is called when the object goes out of scope.
    pub fn dealloc(&mut self, id: usize) {
        self.recycled.push(id);
    }
}
