use std::cell::UnsafeCell;

fn new(parent: Option<&mut Foo>) -> Foo {
    unsafe {
        match parent {
            Some(p) => {
                let counter = *p.counter.get();
                *counter += 1;
                println!("Created a new record, the count is now: {}", *counter);
                Foo {
                    counter: UnsafeCell::new(counter),
                }
            }
            None => {
                let counter = Box::into_raw(Box::new(0));
                println!("counter record: {}", *counter);
                Foo {
                    counter: UnsafeCell::new(counter),
                }
            }
        }
    }
}
