/* SPDX-License-Identifier: MIT                          */
/* LunarOS - x86_64 内核链接脚本                          */
/* 内核加载地址：1MB（0x100000）                          */

ENTRY(_start)

SECTIONS {
    /* 内核起始地址：1MB */
    . = 0x100000;

    /* 代码段：可执行，只读 */
    .text : {
        *(.text .text.*)
    }

    /* 只读数据段 */
    .rodata : {
        *(.rodata .rodata.*)
    }

    /* 已初始化数据段 */
    .data : {
        *(.data .data.*)
    }

    /* 未初始化数据段（BSS） */
    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        _bss_end = .;
    }

    /* 丢弃不需要的段 */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}
