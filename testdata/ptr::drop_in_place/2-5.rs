unsafe fn deallocate_instance(&mut self) {
    let instance_ptr = self.instance.as_ptr();

    ptr::drop_in_place(instance_ptr);
    std::alloc::dealloc(instance_ptr as *mut u8, self.instance_layout);
}
// https://github.com/near/nearcore/blob/9cb316c697b9677611cf49ba5107eb71472c31f0/runtime/near-vm/vm/src/instance/ref.rs#L45