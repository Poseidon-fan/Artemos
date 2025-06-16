.attribute arch, "rv64gc"
FP_START = 51
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

.section .text
    .globl __trap_from_user
    .globl __return_to_user
    .align 2

# user trap into kernel
__trap_from_user:
    # swap sp and sscratch
    # now sp->*TrapContext in user space, sscratch->user stack
    csrrw sp, sscratch, sp
    # save general purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    .set n, 3
    .rept 29
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it in TrapContext
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # load trap_handler into t0 
    ld t0, 35*8(sp)
    # move to kernel sp
    ld sp, 34*8(sp)
    jr t0

# return from kernel trap
__return_to_user:
    # a0: *TrapContext in user space(Constant); a1: user space token
    csrw sscratch, a0
    mv sp, a0

    # now sp points to TrapContext in user space, start restoring based on it
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
    # back to user stack
    ld sp, 2*8(sp)
    sret
