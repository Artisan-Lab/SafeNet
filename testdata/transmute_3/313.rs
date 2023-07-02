pub fn new(num_cores: usize) -> Stats {
        let smp_load: [AtomicUsize; MAX_CORE] = {
            let mut data: [MaybeUninit<AtomicUsize>; MAX_CORE] =
                unsafe { MaybeUninit::uninit().assume_init() };

            for core_data in data.iter_mut().take(num_cores) {
                unsafe {
                    std::ptr::write(core_data.as_mut_ptr(), AtomicUsize::new(0));
                }
            }
            for core_data in data.iter_mut().take(MAX_CORE).skip(num_cores) {
                unsafe {
                    std::ptr::write(core_data.as_mut_ptr(), AtomicUsize::new(usize::MAX));
                }
            }

            unsafe { std::mem::transmute::<_, [AtomicUsize; MAX_CORE]>(data) }
        };
        Stats {
            smp_load,
            mean_level: AtomicUsize::new(0),
            updating_mean: AtomicBool::new(false),
        }
    }
