#[inline(always)]
pub fn distance_absolute<T: Sized>(a: *const T, b: *const T) -> usize {
    unsafe { a.offset_from(b).abs() as usize }
}
//https://github.com/GetFirefly/firefly/blob/8e89bc7ec33cb8ffa9a60283c8dcb7ff62ead5fa/library/system/src/mem/mod.rs#L47