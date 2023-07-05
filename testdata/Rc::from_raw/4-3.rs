pub fn eval(self, app_data: &mut D) -> EvalResult<D> {
    use std::cell::Cell;
    use std::rc::Rc;

    let ptr = Rc::into_raw(Rc::new(app_data));

    // as I am playing around with pointers here, I am going to do assertions in the rebuilding
    // if from_raw is called more than once, it is memory unsafe, so count the calls and assert it is only 1
    let rebuilds: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    let func = || {
        let b = Rc::clone(&rebuilds);

        let n = b.get();

        assert_eq!(n, 0, "unsafe memory operation, can only rebuild Rc once.");

        b.set(n + 1);

        let c = unsafe { Rc::from_raw(ptr) };

        Rc::try_unwrap(c)
            .map_err(|_| "there should only be one strong reference")
            .unwrap()
    };

    map_variants(self, func, func)
}