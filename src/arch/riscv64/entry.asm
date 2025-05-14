    .section .text.entry
    .globl _start
_start:
    # a0 == hartid
    # pc == 0x80200000
    # sp == 0x800xxxxx

    # 1. set sp
    # sp = bootstack + (hartid + 1) * 0x10000
    add     t0, a0, 1
    slli    t0, t0, 16 # 64KB, max stack size
    la      sp, bootstack
    add     sp, sp, t0

    # 2. jump to kernel_main
    call kernel_main

    .section .bss.stack
    .align 12   # page align
    .global bootstack
bootstack:
    .space 4096 * 16 * 8 # 64KB x 8 CPUs
    .global bootstacktop
bootstacktop:

    .section .data
    .align 12   # page align
