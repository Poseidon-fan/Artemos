#[repr(C)]
pub struct ThreadContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl ThreadContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
}
