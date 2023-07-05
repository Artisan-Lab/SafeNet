pub(super) fn new<F: FnOnce()>(f: F) -> Self {
    let size = mem::size_of::<F>();
    let align = mem::align_of::<F>();

    unsafe {
        if size <= mem::size_of::<Data>()
            && align <= mem::align_of::<Data>()
        {
            let mut data = Data::uninit();
            ptr::write(data.as_mut_ptr() as *mut F, f);

            Deferred {
                call: call_raw::<F>,
                data,
                _marker: PhantomData,
            }
        } else {
            let b: Box<F> = Box::new(f);
            let mut data = Data::uninit();
            ptr::write(data.as_mut_ptr() as *mut Box<F>, b);

            Deferred {
                call: call_raw_box::<F>,
                data,
                _marker: PhantomData,
            }
        }
    }
}