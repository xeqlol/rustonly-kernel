ENTRY(_start)

SECTIONS {
    . = 1M;

    .boot ALIGN(4K) :
    {
        KEEP(*(.multiboot_header))
    }

    .text ALIGN(4K) :
    {
        KEEP(*(.text._start))
        *(.text .text.*)
    }
}