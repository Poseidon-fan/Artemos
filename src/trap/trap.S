.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
		# 交换 sp 和 sscratch
		# 现在 sp 指向用户地址空间中的 Trap 上下文结构体
		# sscratch 指向了原来的用户栈
    csrrw sp, sscratch, sp
    # 在 TrapContext 里保存所有寄存器的值
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they have been saved in TrapContext
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # t2 保存了用户栈栈顶（x2 就是 sp，别忘了我们还没保存进入到 __alltraps 之前的 sp）
    csrr t2, sscratch
    # 保存 x2
    sd t2, 2*8(sp)
    # 读取 kernel_satp
    ld t0, 34*8(sp)
    # 读取 trap_handler 入口点
    ld t1, 36*8(sp)
    # 将 sp 修改为应用内核栈顶的地址
    ld sp, 35*8(sp)
    #! 以上为用户地址空间
    csrw satp, t0 #! 这之后指令地址必须在切换前后都有效，所以引出了跳板
    #! 以下为内核地址空间
    sfence.vma
    # 进入到 trap_handler
    # 注意这里没有使用 call，也就是说没有修改 ra
    jr t1

__restore:
    # a0: *TrapContext in user space(Constant); a1: user space token
    # 切换到用户地址空间（为什么先切换：因为 a0 存的是 TrapContext 在应用地址空间中的位置）
    csrw satp, a1
    sfence.vma
    # 将用户地址空间中上下文结构体的地址写入 sscratch（这样 __alltraps 的时候才能找到）
    csrw sscratch, a0
    # 让 sp 指向用户地址空间中上下文结构体
    mv sp, a0
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general purpose registers except x0/sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # 最后再恢复用户栈
    ld sp, 2*8(sp)
    # 返回到用户态
    sret