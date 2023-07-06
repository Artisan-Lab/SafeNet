pub(crate) fn safe_array_from_com_slice(slice: &[IUnknown]) -> *mut SAFEARRAY {
    let sa = unsafe { SafeArrayCreateVector(VT_UNKNOWN, 0, slice.len().try_into().unwrap()) };
    if sa.is_null() {
        panic!("SAFEARRAY allocation failed");
    }
    for (i, item) in slice.iter().enumerate() {
        let i: i32 = i.try_into().unwrap();
        unsafe { SafeArrayPutElement(&*sa, &i, std::mem::transmute_copy(item)) }.unwrap();
    }
    sa
}
// https://github.com/AccessKit/accesskit/blob/3f556c995e8c5eae6831a89b0173809134c1b4e7/platforms/windows/src/util.rs#L172