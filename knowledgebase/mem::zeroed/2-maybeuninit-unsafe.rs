use std::mem;
use std::ptr;
use std::mem::MaybeUninit;
use std::mem::ManuallyDrop;

#[derive(Debug)]
struct MyStruct<'a> {
    number: i32,
    flag: bool,
    b: &'a Box<i32>,
}

fn foo<'a>(x: *mut MyStruct<'a>, y: &'a Box<i32>) {
    unsafe { (*x).b = y; }
}

fn main() {
    let mut myst: ManuallyDrop<MyStruct> = ManuallyDrop::new(unsafe {
        let number = MaybeUninit::zeroed();
        let flag = MaybeUninit::zeroed();
        let b: *const Box<i32> = std::ptr::null(); 

        MyStruct {
            number: number.assume_init(),
            flag: flag.assume_init(),
            b: unsafe { &*b },
        }
    });

    let b = Box::new(1);
    foo(&mut *myst as *mut _, &b);
    println!("{:?}", unsafe { &*myst });
}
