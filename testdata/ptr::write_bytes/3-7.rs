unsafe fn copy_row_up(info: &FramebufferInfo, rows_up: u32) {
    let ptr = match usize::try_from(info.address) {
        Ok(p) => p as *mut u8,
        _ => return,
    };

    let width = match usize::try_from(info.width) {
        Ok(w) => w,
        _ => return,
    };

    let height = match usize::try_from(info.height) {
        Ok(h) => h,
        _ => return,
    };

    let rows_up = match usize::try_from(rows_up) {
        Ok(0) => return,
        Ok(r) => r,
        _ => return,
    };

    let pitch = match usize::try_from(info.pitch) {
        Ok(p) => p,
        _ => return,
    };

    let bpp = usize::from(info.bytes_per_character);

    for y in 0..(height - rows_up) {
        let src = ptr.add(pitch.saturating_mul(y + rows_up));
        let dst = ptr.add(pitch.saturating_mul(y));
        // Note: we don't use `copy_non_overlapping` as we don't actually know if that's true.
        ptr::copy(src, dst, bpp.saturating_mul(width));
    }

    for y in (height - rows_up)..height {
        let dst = ptr.add(pitch.saturating_mul(y));
        ptr::write_bytes(dst, 0x0, bpp.saturating_mul(width));
    }
}
// https://github.com/tomaka/redshirt/blob/c9435212d2dc0f9621696a7df1696b234659ad78/kernel/standalone/src/klog/video.rs#L234