pub fn new<'a>(mem: &MemoryPtr) -> &'a Markers {
    let values = unsafe { mem.read_mut_exact(0, Self::MARKER_SIZE * Self::NUMBER_OF_MARKERS) };
    let (_prefix, shorts, _suffix) = unsafe { values.align_to::<Markers>() };
    &shorts[0]
}
