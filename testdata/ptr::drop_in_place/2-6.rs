unsafe fn finalize(&mut self) {
    ptr::drop_in_place(self as *mut Self)
}
// https://github.com/withoutboats/shifgrethor/blob/ab1873fc3e76e6b9fd9a96b11746617e14bd7c1c/src/lib/gc/trace.rs#L67