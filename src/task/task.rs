use super::TaskContext;
use crate::config::TRAP_CONTEXT;
use crate::mm::{KERNEL_SPACE, MemorySet, PhysPageNum, VirtAddr};
use crate::sync::UPSafeCell;
use crate::trap::{TrapContext, trap_handler};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cell::RefMut;
use crate::task::pid::{KernelStack, PidHandle};

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready, // 准备运行
    Running, // 正在运行
    Exited, // 已退出
    Zombie, // 待回收
}

pub struct TaskControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub trap_cx_ppn: PhysPageNum,
    #[allow(unused)]
    pub base_size: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
}

impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // 从 ELF 里获取应用地址空间、用户栈地址、程序入口点
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        // 找到 trap 上下文被实际放在哪一页帧（内核用了用户的地址空间）
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let task_status = TaskStatus::Ready;
        // 找到该应用的内核栈应该被放到内核地址空间的哪里
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        // 将该应用的内核栈逻辑段插入地址空间
        KERNEL_SPACE
            .exclusive_access()
            .insert_framed_area(
                kernel_stack_bottom.into(),
                kernel_stack_top.into(),
                MapPermission::R | MapPermission::W,
            );
        // 创建任务控制块实例
        let task_control_block = Self {
            task_status,
            // 在应用的内核栈顶压入一个跳转到 trap_return 的任务上下文
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
        };
        // 查找该应用的 Trap 上下文的内核虚地址，其实相当于这个上下文的真实物理地址
        // 这里就体现了内核地址空间的第五个逻辑段为啥要恒等映射
        // 同时在这里我们加入了 TrapContext 的三个新字段
        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }

    // 获得在用户空间的 Trap 上下文的可变引用
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    // 获得用户地址空间对应的 token
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus::Zombie
    }
}