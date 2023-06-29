pub fn new() -> Arc<Self> {
    let mut task_collection = Arc::new(TaskCollection {
        future_collections: Vec::with_capacity(MAX_PRIORITY),
        task_num: AtomicUsize::new(0),
        generator: None,
    });
    // SAFETY: no other Arc or Weak pointers
    let tc_clone = task_collection.clone();
    let mut tc = unsafe { Arc::get_mut_unchecked(&mut task_collection) };
    for priority in 0..MAX_PRIORITY {
        tc.future_collections
            .push(RefCell::new(FutureCollection::new(priority)));
    }
    tc.generator = Some(Mutex::new(Box::pin(TaskCollection::generator(tc_clone))));
    task_collection
}
/*
https://github.com/OSLab-zCore/PreemptiveScheduler/blob/99d7bab1d2eb7045d470b17dca4b59b5b037225d/src/task_collection.rs#L151
*/