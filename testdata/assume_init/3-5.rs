unsafe fn get_region_size(handle: HANDLE, ptr: LPVOID) -> Result<usize, &'static str> {
    let mut meminfo = MaybeUninit::<MEMORY_BASIC_INFORMATION>::uninit();
    if VirtualQueryEx(
        handle,
        ptr,
        meminfo.as_mut_ptr() as *mut _,
        size_of::<MEMORY_BASIC_INFORMATION>(),
    ) == 0
    {
        return Err("Unable to read process memory information");
    }
    let meminfo = meminfo.assume_init();
    Ok((meminfo.RegionSize as isize - ptr.offset_from(meminfo.BaseAddress)) as usize)
}
