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

    adr x0, _start 
    adr x1, _rela_start
    adr x2, _rela_end
    bl _relocate_binary

    # Call into main
    bl main


.equ R_AARCH64_RELATIVE, 1027
_relocate_binary:
    ldp x7, x8, [x1], 16 // Load pair into x7 and x8 from _rela_start and increment by 16
    ldr x9, [x1], 8      // Load the next 8 bytes

    cmp x8, R_AARCH64_RELATIVE //Compare with the value
    bne 1f                     // If not, we return

    add x10, x0, x7   // Add offset to base (_start)
    add x11, x0, x9   // Add addend + sym to base (see readelf output)
    str x11, [x10]     // Store the relocated address back
    cmp x1, x2
    bne _relocate_binary

1: 
    ret