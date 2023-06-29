pub fn create_memory_trap_for_save_memory() {
    unsafe { ORIGINAL_VTTBR_EL2 = cpu::get_vttbr_el2() };
    let page_table = remake_stage2_page_table().expect("Failed to remake page table.");
    cpu::set_vttbr_el2(page_table as u64);
    let list = unsafe { MEMORY_SAVE_LIST.assume_init_read() };
    for e in list {
        if e.num_of_pages == 0 && e.memory_start == 0 {
            break;
        }
        if e.saved_address == MEMORY_SAVE_ADDRESS_ONDEMAND_FLAG {
            /* OnDemand Save */
            add_memory_access_trap(
                e.memory_start,
                (e.num_of_pages as usize) << PAGE_SHIFT,
                true,
                false,
            )
            .expect("Failed to add memory trap");
        }
    }
}
// https://github.com/RIKEN-RCCS/MilvusVisor/blob/fb1e71b10bca2eed23ddbf88f2d47425c97e5b55/src/hypervisor_kernel/src/fast_restore.rs#L60
