.section .text._start
.global _start
.type _start, @function

_start:
    # Initialize the stack
    adr x7, {}       // Move address of STACK into x7
    mov x8, {}       // Move size of STACK into x8
    add x7, x7, x8   // Adjust stack pointer to end of memory region
    mov sp, x7       // Now set the stack pointer

    # Enable FP in register
    mrs x7, cpacr_el1
    orr x7, x7, #(3 << 20)
    msr cpacr_el1, x7

    # Call into main
    bl main
