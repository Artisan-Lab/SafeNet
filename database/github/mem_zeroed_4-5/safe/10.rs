pub fn start<P: AsRef<Path>>(p: P) -> Result<Heap> {

    let mut slabs = vec![];

    for slab_id in 0..32 {
        let slab = Slab::start(&p, slab_id)?;

        slabs.push(slab);
    }

    let slabs: [Slab; 32] = slabs.try_into().unwrap();

    Ok(Heap { slabs })
}