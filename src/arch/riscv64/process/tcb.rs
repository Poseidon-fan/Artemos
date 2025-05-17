pub struct ThreadControlBlock {
    tid: Tid,
}

#[derive(Clone, Copy, Debug)]
struct Tid(usize);

impl ThreadControlBlock {
    fn get_tid(&self) -> Tid {
        self.tid
    }
}
