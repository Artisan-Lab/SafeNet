use std::rc::{Rc, Weak};

let strong = Rc::new("hello".to_owned());

let weak_1 = Rc::downgrade(&strong);
let weak_2 = Rc::downgrade(&strong);

assert_eq!(2, Rc::weak_count(&strong));

assert_eq!("hello", weak_1.upgrade().unwrap());
assert_eq!(1, Rc::weak_count(&strong));

drop(strong);

assert!(weak_2.upgrade().is_none());
