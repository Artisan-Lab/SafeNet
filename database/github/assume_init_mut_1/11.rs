pub fn init(multiboot_ptr: *const c_void) {
	let boot_info = multiboot::get_boot_info();
	let mem_info = unsafe { MEM_INFO.assume_init_mut() };

	mem_info.memory_maps_size = boot_info.memory_maps_size;
	mem_info.memory_maps_entry_size = boot_info.memory_maps_entry_size;
	mem_info.memory_maps = boot_info.memory_maps;

	let (main_begin, main_pages) = get_phys_main(multiboot_ptr);
	mem_info.phys_main_begin = main_begin;
	mem_info.phys_main_pages = main_pages;

	// Setting memory stats
	let mut mem_info = stats::MEM_INFO.lock();
	mem_info.mem_total = min(boot_info.mem_upper, 4194304) as _;
	mem_info.mem_free = main_pages * 4;
}

// https://github.com/llenotre/maestro/blob/acaf4782782cc27ab2c4812c565175e198d8754f/src/memory/memmap.rs#L36