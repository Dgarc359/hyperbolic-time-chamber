OUTPUT_FORMAT("elf32-littleriscv")
ENTRY(_entrypoint)
MEMORY {
    RAM (rw): ORIGIN = 0x80200000, LENGTH = 8M
}

SECTIONS {
    .text : {
        *(.text.entrypoint)
    } >RAM
    .text : {
        *(.text .text.* .gnu.linkonce.t.*)
        *(.rodata .rodata.* .gnu.linkonce.r.*)
        . = ALIGN(4);
        _data_start = .;
        *(.data .data.* .gnu.linkonce.d.*)
        _data_end = .;
        _bss_start = .;
        . = ALIGN(4);
        *(.bss .bss.* .gnu.linkonce.b.*)
        . = ALIGN(4);
        _bss_end = .;
    } >RAM
}
