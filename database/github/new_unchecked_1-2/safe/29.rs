pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'a,
    {
        let boxed: Box<dyn Future<Output = T> + Send + 'a> = Box::new(future);


        let boxed = NonNull::from(Box::leak(boxed));
        Self { boxed }
    }