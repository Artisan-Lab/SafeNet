use std::rc::Rc;

let last = Rc::new(1);
let weak = Rc::downgrade(&last);

let mut v = vec![Rc::new(0), last];

// Move the last element out of the vector, which will prevent it from being dropped.
let last = v.pop().unwrap();

// Drop the last element manually to ensure that its memory is properly freed.
drop(last);

assert_eq!(v, &[0.into()]);

// Ensure that the last item was dropped.
assert!(weak.upgrade().is_none());
