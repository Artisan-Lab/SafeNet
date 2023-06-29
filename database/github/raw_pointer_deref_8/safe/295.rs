use std::sync::{Arc, atomic::{AtomicPtr, Ordering}};

fn new(parent: Option<Arc<Foo>>) -> Foo {
    let counter = match parent {
        Some(p) => {
            let ptr = p.counter.load(Ordering::SeqCst);
            let new_ptr = Box::into_raw(Box::new(*ptr + 1));
            println!("Created a new record, the count is now: {}", *new_ptr);
            new_ptr
        }
        None => {
            let counter = Box::into_raw(Box::new(0));
            println!("counter record: {}", *counter);
            counter
        }
    };

    Foo {
        counter: AtomicPtr::new(counter),
    }
}
