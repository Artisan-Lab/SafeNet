pub fn into_block(self, end_insn_idx: IseqIdx, start_addr: CodePtr, end_addr: CodePtr, gc_obj_offsets: Vec<u32>) -> BlockRef {
    // Allocate the block and get its pointer
    let blockref: *mut MaybeUninit<Block> = Box::into_raw(Box::new(MaybeUninit::uninit()));

    incr_counter_by!(num_gc_obj_refs, gc_obj_offsets.len());

    // Make the new block
    let block = MaybeUninit::new(Block {
        start_addr,
        iseq: Cell::new(self.get_iseq()),
        iseq_range: self.get_starting_insn_idx()..end_insn_idx,
        ctx: self.get_starting_ctx(),
        end_addr: Cell::new(end_addr),
        incoming: MutableBranchList(Cell::default()),
        gc_obj_offsets: gc_obj_offsets.into_boxed_slice(),
        entry_exit: self.get_block_entry_exit(),
        cme_dependencies: self.method_lookup_assumptions.into_iter().map(Cell::new).collect(),
        // Pending branches => actual branches
        outgoing: self.pending_outgoing.into_iter().map(|pending_out| {
            let pending_out = Rc::try_unwrap(pending_out)
                .ok().expect("all PendingBranchRefs should be unique when ready to construct a Block");
            pending_out.into_branch(NonNull::new(blockref as *mut Block).expect("no null from Box"))
        }).collect()
    });
    // Initialize it on the heap
    // SAFETY: allocated with Box above
    unsafe { ptr::write(blockref, block) };

    // Block is initialized now. Note that MaybeUnint<T> has the same layout as T.
    let blockref = NonNull::new(blockref as *mut Block).expect("no null from Box");

    // Track all the assumptions the block makes as invariants
    if self.block_assumes_single_ractor {
        track_single_ractor_assumption(blockref);
    }
    for bop in self.bop_assumptions {
        track_bop_assumption(blockref, bop);
    }
    // SAFETY: just allocated it above
    for cme in unsafe { blockref.as_ref() }.cme_dependencies.iter() {
        track_method_lookup_stability_assumption(blockref, cme.get());
    }
    if let Some(idlist) = self.stable_constant_names_assumption {
        track_stable_constant_names_assumption(blockref, idlist);
    }

    blockref
}
