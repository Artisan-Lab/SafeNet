fn drop(&mut self) {
    assert!(!self.link.is_linked());
    // Check if this fragment needs to have a destructor run
    if let Some(mut destructor) = self.destructor.take() {
        let dtor = unsafe { destructor.as_mut() };
        dtor(self.raw.base);
        unsafe {
            ptr::drop_in_place(dtor);
        }
    }

    // Deallocate the memory backing this fragment
    let (layout, _offset) = Layout::new::<Self>().extend(self.raw.layout()).unwrap();
    unsafe {
        let ptr = NonNull::new_unchecked(self as *const _ as *mut u8);

        Global.deallocate(ptr, layout);
    }
}
// https://github.com/GetFirefly/firefly/blob/8e89bc7ec33cb8ffa9a60283c8dcb7ff62ead5fa/library/alloc/src/fragment/mod.rs#L137