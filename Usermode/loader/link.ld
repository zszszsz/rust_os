ENTRY(start)

SECTIONS {
	. = BASE;
	.text : {
		*(HEADER);
		*(.text .text.*)
	}
	.rodata ALIGN(PAGE_SIZE): {
		rodata_start = .;
		*(.rodata .rodata.*)
	}
	/* ARM unwind tables (not present) */
	/DISCARD/ : { *(.ARM.extab.init .ARM.exidx.init) }
	.ARM.extab : {
		*(.ARM.extab* .gnu.linkonce.armextab.*)
	}
	PROVIDE_HIDDEN ( __exidx_start = . );
	.ARM.exidx : {
		*(.ARM.exidx* .gnu.linkonce.armexidx.*)
	}
	PROVIDE_HIDDEN ( __exidx_end = . );
	/*
	PROVIDE_HIDDEN ( __exidx_start = . );
	PROVIDE_HIDDEN ( __exidx_end = . );
	/* */

	. = ALIGN(PAGE_SIZE);
	codesize = . - BASE;
	.data ALIGN(PAGE_SIZE): {
		data_start = .;
		*(.data .data.*)
		*(.got .got.*)
		. = ALIGN(PAGE_SIZE);
	} = 0x0000
	.bss ALIGN(PAGE_SIZE) : {
		bss_start = .;
		*(.bss .bss.*)
		bss_end = .;
	}
	imagesize = . - BASE;

	ASSERT(. < LIMIT, "Image is too large to fit in reservation")
}
