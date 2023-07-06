fn drop(&mut self) {
    unsafe {
        let len = self.read_len();
        if len != 0 {
            let slice = ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), len);
            ptr::drop_in_place(slice);
            let alloc = self.ptr.cast::<usize>().as_ptr().sub(1);
            alloc::dealloc(alloc as *mut u8, Self::layout_for_len(len));
        }
    }
}
// https://github.com/facebook/buck2/blob/1835e7fd21530ff58ad9f690e8723d4e13131462/app/buck2_util/src/thin_box.rs#L137