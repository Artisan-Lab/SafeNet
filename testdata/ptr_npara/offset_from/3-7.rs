pub fn iseq_pc_to_insn_idx(iseq: IseqPtr, pc: *mut VALUE) -> Option<u16> {
    let pc_zero = unsafe { rb_iseq_pc_at_idx(iseq, 0) };
    unsafe { pc.offset_from(pc_zero) }.try_into().ok()
}