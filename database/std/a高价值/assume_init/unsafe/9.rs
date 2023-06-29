#![allow(unused)]
#![feature(new_uninit)]

fn main() {
    let mut values = Box::<[u32]>::new_uninit_slice(3);

    let values = unsafe {
        // Deferred initialization:
        values[0].as_mut_ptr().write(1);
        values[1].as_mut_ptr().write(2);
        values[2].as_mut_ptr().write(3);

        values.assume_init()
    };

    assert_eq!(*values, [1, 2, 3]);
}