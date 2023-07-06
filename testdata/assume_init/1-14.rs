pub fn array_init<F, T, const N: usize>(mut initializer: F) -> [T; N]
where
    F: FnMut(usize) -> T,
{
    let mut array: core::mem::MaybeUninit<[T; N]> = core::mem::MaybeUninit::uninit();
    let mut ptr_i = array.as_mut_ptr() as *mut T;

    unsafe {
        for i in 0..N {
            let value_i = initializer(i);
            ptr_i.write(value_i);
            ptr_i = ptr_i.add(1);
        }
        array.assume_init()
    }
}