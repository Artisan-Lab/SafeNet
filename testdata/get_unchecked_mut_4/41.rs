unsafe fn insert<A: HalApi>(
    start_states: Option<&mut [BufferUses]>,
    current_states: &mut [BufferUses],
    index: usize,
) {
    let new_start_state = unsafe { start_state_provider.get_state(index) };
    let new_end_state =log::trace!("\tbuf {index}: insert {new_start_state:?}..{new_end_state:?}");

    unsafe {
        if let Some(&mut ref mut start_state) = start_states {
            *start_state.get_unchecked_mut(index) = new_start_state;
        }
        *current_states.get_unchecked_mut(index) = new_end_state;
    }
}
/*
https://github.com/gfx-rs/wgpu/blob/45efae315bf182ffbe9d6485c4b411ecacf05f33/wgpu-core/src/track/buffer.rs#L717
*/