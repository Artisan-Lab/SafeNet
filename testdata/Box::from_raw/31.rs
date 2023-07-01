fn make_fixed_size<T>(buffer: Box<[T]>) -> Box<[T; LOCAL_QUEUE_CAPACITY]> {
    assert_eq!(buffer.len(), LOCAL_QUEUE_CAPACITY);

    // safety: We check that the length is correct.
    unsafe { Box::from_raw(Box::into_raw(buffer).cast()) }
}
/*
https://github.com/tokio-rs/tokio/blob/2e62374e4a643015ea9ecf5a1d012c3429eed42f/tokio/src/runtime/scheduler/multi_thread/queue.rs#L81
*/