use std::ptr::NonNull;
pub fn kfree(ptr: usize) {
    let mut head = KMEM.lock();
    // we need to convert frame to mutable
    unsafe {
        let mut frame: NonNull<Frame> = Frame::new(ptr);
        frame.as_mut().set(head.take_next());
        head.set(Some(frame));
    }
}
/*
https://github.com/ysj1173886760/TinyOS/blob/8751acb2b38e7b310b2e4d4896a4d8c86b58659e/TinyOS/src/mm/kalloc.rs#L60
*/