// https://github.com/GothenburgBitFactory/taskwarrior/blob/5db8b292c726f2ac53d61e37e0ffe92ecb65d68e/taskchampion/lib/src/traits.rs#L48

fn take_val_from_arg(arg: *mut Self, mut replacement: Self) -> Self::RustType {
    // SAFETY:
    //  - arg is valid (promised by caller)
    //  - replacement is valid and aligned (guaranteed by Rust)

    //  safe：let arg_ref = unsafe { &mut *arg };
    unsafe { std::ptr::swap(arg, &mut replacement) };
    // SAFETY:
    //  - replacement (formerly *arg) is a valid CType (promised by caller)
    unsafe { PassByValue::val_from_arg(replacement) }
}