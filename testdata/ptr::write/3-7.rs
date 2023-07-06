unsafe fn set_process_function(
    &self,
    accessible_memory_start: *const u8,
    app_brk: *const u8,
    state: &mut CortexMStoredState,
    callback: kernel::process::FunctionCall,
) -> Result<(), ()> {
    // Ensure that [`state.psp`, `state.psp + SVC_FRAME_SIZE`] is within
    // process-accessible memory. Alignment is guaranteed by hardware.
    if state.psp < accessible_memory_start as usize
        || state.psp.saturating_add(SVC_FRAME_SIZE) > app_brk as usize
    {
        return Err(());
    }

    // Notes:
    //  - Instruction addresses require `|1` to indicate thumb code
    //  - Stack offset 4 is R12, which the syscall interface ignores
    let stack_bottom = state.psp as *mut usize;
    ptr::write(stack_bottom.offset(7), state.psr); //......... -> APSR
    ptr::write(stack_bottom.offset(6), callback.pc | 1); //... -> PC
    ptr::write(stack_bottom.offset(5), state.yield_pc | 1); // -> LR
    ptr::write(stack_bottom.offset(3), callback.argument3); // -> R3
    ptr::write(stack_bottom.offset(2), callback.argument2); // -> R2
    ptr::write(stack_bottom.offset(1), callback.argument1); // -> R1
    ptr::write(stack_bottom.offset(0), callback.argument0); // -> R0

    Ok(())
}
// https://github.com/tock/tock/blob/6e0aeb0328a114775ce301bac48682173a54a4bd/arch/cortex-m/src/syscall.rs#L234