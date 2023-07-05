/*
https://github.com/WasmEdge/hyper/blob/1fcc180a4f4eba2bc0253f5a11841ccf32732cc2/src/proto/h1/role.rs#L2740
*/
 fn restart(b: &mut BytesMut, len: usize) {
            b.reserve(1);
            unsafe {
                b.set_len(len);
            }
        }
    }