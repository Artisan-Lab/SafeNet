/*
https://github.com/turboladen/overkill_nvim/blob/836b229379c90be0602304f2c19d422a790b0b02/neovim_sys/src/api/nvim/string.rs#L100
*/
use std::mem::MaybeUninit;
fn new(cstring: CString) -> String {
    let mut vec = cstring.into_bytes();

    let mut uninit: MaybeUninit<String> = MaybeUninit::uninit();
    let ptr = uninit.as_mut_ptr();

    unsafe {
        addr_of_mut!((*ptr).size).write(vec.len());
    }

    let new_data = vec.as_mut_ptr().cast::<c_char>();

    unsafe {
        addr_of_mut!((*ptr).data).write(new_data);
    }
    mem::forget(vec);

    unsafe { uninit.assume_init() }
}