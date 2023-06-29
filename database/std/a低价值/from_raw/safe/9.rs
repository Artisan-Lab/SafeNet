use std::sync::{Arc, Weak};

let strong = Arc::new("hello".to_owned());
let weak1 = Arc::downgrade(&strong);
let weak2 = Arc::downgrade(&strong);

assert_eq!(2, Arc::weak_count(&strong));

assert_eq!("hello", &*weak1.upgrade().unwrap());
assert_eq!(1, Arc::weak_count(&strong));

drop(strong);

// Decrement the last weak count.
assert!(weak2.upgrade().is_none());
