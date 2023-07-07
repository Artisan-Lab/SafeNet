pub unsafe fn new_unchecked(ptr: *mut T) -> Self {
    NonZeroPtr(&*ptr)
}
// https://github.com/chansuke/servo/blob/82b3d1043c373c580c6344c4fe72e24018c15c6e/components/hashglobe/src/shim.rs#L8