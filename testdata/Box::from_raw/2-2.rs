pub unsafe fn free(arena: *mut Self) {
    let arena = Box::from_raw(arena);
    std::mem::drop(arena);
}

// https://github.com/xodiumluma/googleprotocolbuffers/blob/c336db79a9a9744c78df0a11492172e9fb7c9f8f/rust/cpp_kernel/cpp.rs#L48