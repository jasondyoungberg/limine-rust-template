/* Tell the linker that we want an x86_64 ELF64 output file */
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

/* We want the symbol _start to be our entry point */
ENTRY(_start)

/* Define the program headers we want so the bootloader gives us the right */
/* MMU permissions */
PHDRS
{
    requests PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    text     PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata   PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data     PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    dynamic  PT_DYNAMIC FLAGS((1 << 1) | (1 << 2)) ; /* Dynamic PHDR for relocations */
}

SECTIONS
{
    /* The bootloader will relocate relocatable kernels with a base address of 0 */
    /* to or above the address 0xffffffff80000000, which marks the beginning of */
    /* the top-most 2GiB region of virtual address space. */
    /* Marking the base address as 0 also makes the GNU linker (ld.bfd) emit an ELF */
    /* file of type ET_DYN, which is what we want. */
    . = 0;

    /* Define a section to contain the Limine requests and assign it to its own PHDR */
    .requests : {
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :requests

    /* Move to the next memory page for .text */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .text : {
        *(.text .text.*)
    } :text

    /* Move to the next memory page for .rodata */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    /* Move to the next memory page for .data */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)
    } :data

    /* Dynamic section for relocations, both in its own PHDR and inside data PHDR */
    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    /* NOTE: .bss needs to be the last thing mapped to :data, otherwise lots of */
    /* unnecessary zeros will be written to the binary. */
    /* If you need, for example, .init_array and .fini_array, those should be placed */
    /* above this. */
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    /* Discard .note.* and .eh_frame since they may cause issues on some hosts. */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}
