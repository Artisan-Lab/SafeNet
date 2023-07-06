fn transmute_box<R>(self: Box<Self>) -> Box<R>
    where
        R: ?Sized + TransmuteBox,
    {
        let value = Box::into_raw(self);
        // SAFETY: This trait is only implemented for types that can be
        // transmuted.
        unsafe { Box::from_raw(mem::transmute_copy(&value)) }
    }

// https://github.com/dylni/os_str_bytes/blob/128b899f28d4da7d3af400bff099cad905be45e3/src/raw_str.rs#L61