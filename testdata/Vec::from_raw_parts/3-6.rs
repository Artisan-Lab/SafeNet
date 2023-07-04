pub fn new(bytes: Vec<u8>) -> Self {
    let capacity = bytes.capacity();

    let dealloc = ConcreteBlock::new(move |bytes: *mut c_void, len: usize| unsafe {
        // Recreate the Vec and let it drop
        let _ = Vec::from_raw_parts(bytes as *mut u8, len, capacity);
    });
    let dealloc = dealloc.copy();
    let dealloc: &Block<(*mut c_void, usize), ()> = &dealloc;

    let mut bytes = bytes;
    let bytes_ptr = bytes.as_mut_ptr() as *mut c_void;

    unsafe {
        let obj: id = msg_send![class!(NSData), alloc];
        let obj: id = msg_send![obj, initWithBytesNoCopy:bytes_ptr
                                                         length:bytes.len()
                                                    deallocator:dealloc];
        mem::forget(bytes);
        NSData(Id::from_ptr(obj))
    }
}
// https://github.com/ryanmcgrath/cacao/blob/d417289f92b5ec13db1608c6e43485f8894b966c/src/foundation/data.rs#L35