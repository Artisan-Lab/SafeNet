fn peek_rc<T>(ctx: &mut Context, idx: i32, copy: bool) -> Option<Rc<T>> {
    ctx.get_object(idx);

    if !ctx.get_prop(idx, "__type") {
        return None;
    }
    let typ = ctx.get_string(-1);
    ctx.pop_it();
    if typ != std::any::type_name::<T>() {
        return None;
    }

    if !ctx.get_prop(idx, "__rc") {
        return None;
    }
    let ptr = ctx.get_pointer(-1);
    ctx.pop_it();
    if copy {
        // increment because we just produced a new Rc and 1 rc is left in stack
        unsafe { Rc::increment_strong_count(ptr) };
    }
    let rc = unsafe { Rc::from_raw(ptr as *const T) };
    Some(rc)
}

/*
https://github.com/polachok/duktape/blob/598117bfdbdae5e67fe1d61f6adaa236bb1982fd/src/value.rs#L132
*/