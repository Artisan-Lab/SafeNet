fn downcast_from(boxed: BlockMetaInfoPtr) -> Option<Self> {
    if boxed.as_any().is::<T>() {
        unsafe {
            // SAFETY: `is` ensures this type cast is correct
            let raw_ptr = Box::into_raw(boxed) as *const dyn BlockMetaInfo;
            return Some(std::ptr::read(raw_ptr as *const Self));
        }
    }

    None
}
// https://github.com/datafuselabs/databend/blob/4c9412d3c40641ac7d03d5e51a90caadb3fa0c4a/src/query/expression/src/block.rs#L96