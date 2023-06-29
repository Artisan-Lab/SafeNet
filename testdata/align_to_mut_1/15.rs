fn unmask_fallback(buf: &mut [u8], mask: [u8; 4]) {
    let mask_u32 = u32::from_ne_bytes(mask);
  
    let (prefix, words, suffix) = unsafe { buf.align_to_mut::<u32>() };
    unmask_easy(prefix, mask);
    let head = prefix.len() & 3;
    let mask_u32 = if head > 0 {
      if cfg!(target_endian = "big") {
        mask_u32.rotate_left(8 * head as u32)
      } else {
        mask_u32.rotate_right(8 * head as u32)
      }
    } else {
      mask_u32
    };
    for word in words.iter_mut() {
      *word ^= mask_u32;
    }
    unmask_easy(suffix, mask_u32.to_ne_bytes());
  }
  