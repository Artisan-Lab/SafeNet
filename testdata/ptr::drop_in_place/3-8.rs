fn do_drop(stack: Pin<&mut Self::StackStorage>) {
    // Switch to MaybeUninit::assume_init_drop when stabilized
    // Safety: per caller guarantees of populate_stack_space, we know this hasn't moved.
    unsafe { std::ptr::drop_in_place(Pin::into_inner_unchecked(stack).assume_init_mut()) };
}
// https://github.com/google/autocxx/blob/bfca1e04675245ed774ce5c73361d1655271d212/src/value_param.rs#L108