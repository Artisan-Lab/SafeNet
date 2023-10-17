fn spec_next_chunk(&mut self) -> Result<[T; N], array::IntoIter<T, N>> {
    let mut raw_array = MaybeUninit::uninit_array();

    let len = self.len();

    if T::IS_ZST {
        if len < N {
            let _ = self.advance_by(len);
            // SAFETY: ZSTs can be conjured ex nihilo; only the amount has to be correct
            return Err(unsafe { array::IntoIter::new_unchecked(raw_array, 0..len) });
        }

        let _ = self.advance_by(N);
        // SAFETY: ditto
        return Ok(unsafe { MaybeUninit::array_assume_init(raw_array) });
    }