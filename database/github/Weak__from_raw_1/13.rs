fn main() {
    use std::rc::{Rc, Weak};

    let strong = Rc::new("hello".to_owned());

    let raw_1 = Rc::downgrade(&strong).into_raw();
    let raw_2 = Rc::downgrade(&strong).into_raw();

    assert_eq!(2, Rc::weak_count(&strong));

    assert_eq!("hello", &*unsafe { Weak::from_raw(raw_1) }.upgrade().unwrap());
    assert_eq!(1, Rc::weak_count(&strong));

    drop(strong);

    assert!(unsafe { Weak::from_raw(raw_2) }.upgrade().is_none());
}


// https://github.com/Artisan-Lab/Rust-SGNN/blob/024233e2fcd98a42c12643ab00b75866d171b341/data/Weak%3A%3Afrom_raw/1-weakfromraw-simple-unsafe.rs#L12