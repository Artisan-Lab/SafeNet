pub unsafe fn unsafe_clone(&self) -> Encoder<'a, W> {
    Encoder {
        writer: mem::transmute_copy(&self.writer),
        size_positions: self.size_positions.clone(),
    }
}

// https://github.com/sdemos/rust/blob/88d8ba5ab3b1d22288b021708c3d87464e43b880/src/librbml/lib.rs#L753