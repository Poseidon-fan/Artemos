use core::arch::asm;

const INIT_CPU_CONTEXT: CpuContext = CpuContext::new();
static mut CPU_CONTEXTS: [CpuContext; 16] = [INIT_CPU_CONTEXT; 16];

/// Represents the data occupied by each CPU.
/// It's addr is contained in `tp` register of each CPU.
/// Every CPU can only access its own `CpuContext` through `tp` register,
/// so it's safe to access `CpuContext` without any synchronization.
#[repr(C)]
#[repr(align(64))]
pub struct CpuContext {
    hart_id: usize,
    enable: bool,
    // ... to be added
}

unsafe impl Sync for CpuContext {}
unsafe impl Send for CpuContext {}


impl CpuContext {
    pub const fn new() -> Self {
        Self {
            hart_id: usize::MAX,
            enable: false,
        }
    }
}

pub fn init_local_cpu_context(hart_id: usize) {
    unsafe {
        // 1. initialize cpu context
        let context = get_mut_cpu_context_by_id(hart_id);
        context.hart_id = hart_id;
        context.enable = true;

        // 2. set tp register to point to cpu context
        asm!("mv tp, {}", in(reg) context as *const CpuContext);
    }
}

#[inline(always)]
unsafe fn get_mut_cpu_context_by_id(hart_id: usize) -> &'static mut CpuContext {
    unsafe { &mut CPU_CONTEXTS[hart_id] }
}


/// Get the hart id of the current CPU.
#[inline(always)]
pub fn hart_id() -> usize {
    let context: *const CpuContext;
    unsafe {
        asm!("mv {}, tp", out(reg) context);
        (*context).hart_id
    }
}
