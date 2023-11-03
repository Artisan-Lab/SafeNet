fn test_import() {
    // Model receiving const pointers from an external system

    // Create an array natively
    let data = Int32Array::from(vec![1, 2, 3]).into_data();
    let schema = FFI_ArrowSchema::try_from(data.data_type()).unwrap();
    let array = FFI_ArrowArray::new(&data);

    // Use ManuallyDrop to avoid Box:Drop recursing
    let schema = Box::new(ManuallyDrop::new(schema));
    let array = Box::new(ManuallyDrop::new(array));

    let schema_ptr = &**schema as *const _;
    let array_ptr = &**array as *const _;

    // We can read them back to memory
    // SAFETY:
    // Pointers are aligned and valid
    let data = unsafe {
        from_ffi(std::ptr::read(array_ptr), &std::ptr::read(schema_ptr)).unwrap()
    };

    let array = Int32Array::from(data);
    assert_eq!(array, Int32Array::from(vec![1, 2, 3]));
}
// https://github.com/apache/arrow-rs/blob/aac3aa99398c4f4fe59c60d1839d3a8ab60d00f3/arrow/src/ffi.rs#L469