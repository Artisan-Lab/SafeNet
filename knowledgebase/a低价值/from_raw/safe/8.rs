use std::sync::Arc;

let x = Arc::new("hello".to_owned());

{
    let x_ptr = Arc::as_ptr(&x);

    // We still have a reference to `x` above, so it won't be dropped and the pointer won't be dangling.

    let x_ref = unsafe { &*x_ptr };
    assert_eq!(x_ref, "hello");
}

// `x` is dropped when it goes out of scope, so we don't need to worry about freeing the memory manually.
