pub fn unsafe_try_cast<A: Any, B: Any>(a: A) -> Result<B, A> {
    if TypeId::of::<B>() == a.type_id() {
        // SAFETY: Just checked we have the right type. We explicitly forget the
        // value immediately after moving out, removing any chance of a destructor
        // running or value otherwise being used again.
        unsafe {
            let ret: B = ptr::read(&a as *const _ as *const B);
            mem::forget(a);
            Ok(ret)
        }
    } else {
        Err(a)
    }
}
// https://github.com/Artisan-Lab/SafeNet/tree/main/knowledgebase