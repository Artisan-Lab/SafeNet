use std::ptr::NonNull;
fn jobject_array_to_vec_of_objects<T: SwigForeignClass + Clone>(
    env: *mut JNIEnv,
    arr: internal_aliases::JForeignObjectsArray<T>,
) -> Vec<T> {
    let field_id = <T>::jni_class_pointer_field();
    let len = <usize as ::std::convert::TryFrom<jsize>>::try_from(length)
        .expect("invalid jsize, in jsize => usize conversion");
    let mut result = Vec::with_capacity(len);
    for i in 0..length {
        let native: &mut T = unsafe {
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Failed to retrieve element {} from this `jobjectArray'", i);
            }
            let ptr = (**env).GetLongField.unwrap()(env, obj, field_id);
            let native = (jlong_to_pointer(ptr) as *mut T).as_mut().unwrap();
            (**env).DeleteLocalRef.unwrap()(env, obj);
            native
        };
        result.push(native.clone());
    }
/*
https://github.com/ijc/flapigen-rs/blob/2628f9d0fad0500412aab06a34077a71812939f7/macroslib/src/java_jni/jni-include.rs#L422
*/