fn heap_available(&self) -> usize {
    unsafe { self.heap_end().offset_from(self.heap_top()) as usize }
}
//https://github.com/GetFirefly/firefly/blob/8e89bc7ec33cb8ffa9a60283c8dcb7ff62ead5fa/library/alloc/src/heap/mod.rs#L128