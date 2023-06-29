use std::sync::Arc;

let mut x = Arc::new(String::new());
Arc::get_mut(&mut x).unwrap().push_str("foo");
assert_eq!(*x, "foo");
