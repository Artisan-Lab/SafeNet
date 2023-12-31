pub fn start<P: AsRef<Path>>(p: P) -> Result<Heap> {
    let mut slabs: [MaybeUninit<Slab>; 32] = unsafe { std::mem::zeroed() };
    for slab_id in 0..32 {
        let slab = Slab::start(&p, slab_id)?;
        slabs[slab_id as usize] = MaybeUninit::new(slab);

    }

    Ok(Heap { slabs: unsafe { transmute(slabs) } })

}