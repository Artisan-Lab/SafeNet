let num_rets = rets_list.len();
                if !using_rets_array && num_rets > 0 {
                    let src_pointer = params_list.as_ptr();
                    let rets_list = &mut rets_list_array.as_mut()[0] as *mut RawValue;
                    unsafe {
                        // TODO: we can probably remove this copy by doing some clever `transmute`s.
                        // we know it's not overlapping because `using_rets_array` is false
                        std::ptr::copy_nonoverlapping(src_pointer,
                                                        rets_list,
                                                        num_rets);
                    }
                }

// https://github.com/wasmerio/wasmer/blob/fca2b07eed002e3cdb0b201016ca2400ac84d279/lib/api/src/sys/typed_function.rs#L85