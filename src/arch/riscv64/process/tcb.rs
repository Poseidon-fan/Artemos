struct ThreadControlBlock {
    tid: Tid,
}

impl ThreadControlBlock {
    fn get_tid(&self) -> Tid {
        self.tid
    }
}
