    .section .text.entry
    .globl _start
_start:
    # a0 = hart id
    # pc = 0x80200000

    # set kernel boot stack
    # stack size: 4 KB * 16
    slli t0, a0, 16
    la sp, boot_stack_top
    sub sp, sp, t0

    # activate paging, mapping kernel to high address
    # satp: 8 << 60 | boot_pagetable
    la t0, boot_pagetable
    li t1, 8 << 60
    srli t0, t0, 12
    or t0, t0, t1
    csrw satp, t0
    sfence.vma

    # call rust_entry
    call rust_entry

    .section .bss.stack

    .globl boot_stack_bottom
boot_stack_bottom:
    .space 4096 * 16 * 8  # 8 CPUS at most

    .globl boot_stack_top
boot_stack_top:

    .section .data
    .align 12
boot_pagetable:
    # 0x0000_0000_8000_0000 -> 0x0000_0000_8000_0000
    # 0xffff_fc00_8000_0000 -> 0x0000_0000_8000_0000
    .quad 0
    .quad 0
    .quad (0x80000 << 10) | 0xcf # VRWXAD
    .zero 8 * 255
    .quad (0x80000 << 10) | 0xcf # VRWXAD
    .zero 8 * 253