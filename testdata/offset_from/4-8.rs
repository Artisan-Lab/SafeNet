pub fn gen_entry_point(iseq: IseqPtr, ec: EcPtr, jit_exception: bool) -> Option<CodePtr> {
    // Compute the current instruction index based on the current PC
    let cfp = unsafe { get_ec_cfp(ec) };
    let insn_idx: u16 = unsafe {
        let ec_pc = get_cfp_pc(cfp);
        iseq_pc_to_insn_idx(iseq, ec_pc)?
    };
    let stack_size: u8 = unsafe {
        u8::try_from(get_cfp_sp(cfp).offset_from(get_cfp_bp(cfp))).ok()?
    };

    // The entry context makes no assumptions about types
    let blockid = BlockId {
        iseq,
        idx: insn_idx,
    };

    // Get the inline and outlined code blocks
    let cb = CodegenGlobals::get_inline_cb();
    let ocb = CodegenGlobals::get_outlined_cb();

    // Write the interpreter entry prologue. Might be NULL when out of memory.
    let code_ptr = gen_entry_prologue(cb, ocb, iseq, insn_idx, jit_exception);

    // Try to generate code for the entry block
    let mut ctx = Context::default();
    ctx.stack_size = stack_size;
    let block = gen_block_series(blockid, &ctx, ec, cb, ocb);

    cb.mark_all_executable();
    ocb.unwrap().mark_all_executable();

    match block {
        // Compilation failed
        None => {
            // Trigger code GC. This entry point will be recompiled later.
            cb.code_gc(ocb);
            return None;
        }

        // If the block contains no Ruby instructions
        Some(block) => {
            let block = unsafe { block.as_ref() };
            if block.iseq_range.is_empty() {
                return None;
            }
        }
    }

    // Count the number of entry points we compile
    incr_counter!(compiled_iseq_entry);

    // Compilation successful and block not empty
    return code_ptr;
}