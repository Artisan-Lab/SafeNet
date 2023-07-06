pub fn new(array: [T; N]) -> Self {
    unsafe {
        let iter = Self { data: mem::transmute_copy(&array), alive: 0..N };
        mem::forget(array);
        iter
    }
}
// https://github.com/mark-i-m/rust/blob/9a700d2947f2d7f97a2c0dfca3117a8dcc255bdd/library/core/src/array/iter.rs#L70