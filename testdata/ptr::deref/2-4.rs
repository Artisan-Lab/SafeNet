pub fn as_ptr(&self) -> *mut T {
    self.0 as *const T as *mut T
}
// https://github.com/chansuke/servo/blob/82b3d1043c373c580c6344c4fe72e24018c15c6e/components/hashglobe/src/shim.rs#L8