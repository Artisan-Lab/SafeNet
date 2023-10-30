fn default() -> Histogram {
    // #[cfg(not(feature = "miri_optimizations"))]
    // {
    //     let mut vals = Vec::with_capacity(BUCKETS);
    //     vals.resize_with(BUCKETS, Default::default);

    //     Histogram {
    //         vals,
    //         sum: AtomicUsize::new(0),
    //         count: AtomicUsize::new(0),
    //     }
    // }

    #[cfg(feature = "miri_optimizations")]
    {
        let mut raw_vals =
            std::mem::ManuallyDrop::new(vec![0_usize; BUCKETS]);
        let ptr: *mut usize = raw_vals.as_mut_ptr();
        let len = raw_vals.len();
        let capacity = raw_vals.capacity();

        let vals: Vec<AtomicUsize> = unsafe {
            Vec::from_raw_parts(ptr as *mut AtomicUsize, len, capacity)
        };

        Histogram {
            vals,
            sum: AtomicUsize::new(0),
            count: AtomicUsize::new(0),
        }
    }
}

// https://github.com/spacejam/sled/blob/69294e59c718289ab3cb6bd03ac3b9e1e072a1e7/src/histogram.rs#L77