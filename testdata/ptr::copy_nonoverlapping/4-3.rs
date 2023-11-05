unsafe fn encode_one(
    out: &mut [MaybeUninit<u8>],
    val: Option<&[MaybeUninit<u8>]>,
    field: &SortField,
) -> usize {
    match val {
        Some(val) if val.is_empty() => {
            let byte = if field.descending {
                !EMPTY_SENTINEL
            } else {
                EMPTY_SENTINEL
            };
            *out.get_unchecked_release_mut(0) = MaybeUninit::new(byte);
            // write remainder as zeros
            out.get_unchecked_release_mut(1..).fill(MaybeUninit::new(0));
            1
        }
        Some(val) => {
            let block_count = ceil(val.len(), BLOCK_SIZE);
            let end_offset = 1 + block_count * (BLOCK_SIZE + 1);

            let dst = out.get_unchecked_release_mut(..end_offset);

            // Write `2_u8` to demarcate as non-empty, non-null string
            *dst.get_unchecked_release_mut(0) = MaybeUninit::new(NON_EMPTY_SENTINEL);

            let src_chunks = val.chunks_exact(BLOCK_SIZE);
            let src_remainder = src_chunks.remainder();

            // + 1 is for the BLOCK CONTINUATION TOKEN
            let dst_chunks = dst
                .get_unchecked_release_mut(1..)
                .chunks_exact_mut(BLOCK_SIZE + 1);

            for (src, dst) in src_chunks.zip(dst_chunks) {
                // we copy src.len() that leaves 1 bytes for the continuation tkn.
                std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
                // Indicate that there are further blocks to follow
                *dst.get_unchecked_release_mut(BLOCK_SIZE) =
                    MaybeUninit::new(BLOCK_CONTINUATION_TOKEN);
            }

            if src_remainder.is_empty() {
                // get the last block
                let start_offset = 1 + (block_count - 1) * (BLOCK_SIZE + 1);
                let last_dst = dst.get_unchecked_release_mut(start_offset..);
                last_dst.fill(MaybeUninit::new(0));

                // overwrite the latest continuation marker.
                // replace the "there is another block" with
                // "we are finished this, this is the length of this block"
                *dst.last_mut().unwrap_unchecked() = MaybeUninit::new(BLOCK_SIZE as u8);
            } else {
                // get the last block
                let start_offset = 1 + (block_count - 1) * (BLOCK_SIZE + 1);
                let last_dst = dst.get_unchecked_release_mut(start_offset..);
                let n_bytes_to_write = src_remainder.len();

                std::ptr::copy_nonoverlapping(
                    src_remainder.as_ptr(),
                    last_dst.as_mut_ptr(),
                    n_bytes_to_write,
                );
                // write remainder as zeros
                last_dst
                    .get_unchecked_release_mut(n_bytes_to_write..last_dst.len() - 1)
                    .fill(MaybeUninit::new(0));
                *dst.last_mut().unwrap_unchecked() = MaybeUninit::new(src_remainder.len() as u8);
            }

            if field.descending {
                for byte in dst {
                    *byte = MaybeUninit::new(!byte.assume_init());
                }
            }
            end_offset
        }
        None => {
            *out.get_unchecked_release_mut(0) = MaybeUninit::new(get_null_sentinel(field));
            // write remainder as zeros
            out.get_unchecked_release_mut(1..).fill(MaybeUninit::new(0));
            1
        }
    }
}
// https://github.com/pola-rs/polars/blob/d6898f7dc1dfcb9b130f89a71a43d39f20458023/polars/polars-row/src/variable.rs#L103