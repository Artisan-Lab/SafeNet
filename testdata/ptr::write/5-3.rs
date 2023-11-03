pub fn new(cache: Box<dyn ObjectCache>) -> Box<Self> {
    let this = Box::new(Self {
        wrapper: Default::default(),
        cache,
    });
    let context = Box::into_raw(this);
    let wrapper = unsafe {
        LLVMObjectCacheWrapperCreate(
            context as *mut c_void,
            object_cache_getter,
            object_cache_notifier,
        )
    };
    let mut this = unsafe { Box::from_raw(context) };
    unsafe { ptr::write(&mut this.wrapper, wrapper) };
    this
}
// https://github.com/GetFirefly/firefly/blob/8e89bc7ec33cb8ffa9a60283c8dcb7ff62ead5fa/compiler/llvm/src/jit.rs#L91