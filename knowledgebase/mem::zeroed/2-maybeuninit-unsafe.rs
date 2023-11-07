use std::mem::MaybeUninit;

#[derive(Debug)]
struct MyStruct<'a> {
    number: i32,
    flag: bool,
    b: Option<&'a Box<i32>>,
}

fn foo<'a>(x: *mut MyStruct<'a>, y: &'a Box<i32>) {
    unsafe { (*x).b = Some(y); }
}

fn main() {
    let mut myst: MyStruct = unsafe {
        let number = MaybeUninit::zeroed();
        let flag = MaybeUninit::zeroed();
        MyStruct {
            number: number.assume_init(),
            flag: flag.assume_init(),
            b: None,
        }
    };

    let b = Box::new(1);
    foo(&mut myst as *mut _, &b);
    println!("{:?}", myst);
}
