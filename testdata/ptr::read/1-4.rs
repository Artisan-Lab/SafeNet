pub extern "C" fn store_rgb8_compressed(
    data: *const u8, len: usize,
    layout: *const SampleLayout
) 
-> bool
{
    let samples = unsafe { slice::from_raw_parts(data, len) };
    let layout = unsafe { ptr::read(layout) };
    let buffer = FlatSamples {
        samples,
        layout,
        color_hint: None,
    };
    let view = match buffer.as_view::<Rgb<u8>>() {
        Err(_) => return false, // Invalid layout.
        Ok(view) => view,
    };
    thumbnail(&view, 64, 64)
    .save("output.png")
    .map(|_| true)
    .unwrap_or_else(|_| false)
}

// https://github.com/image-rs/image/blob/90cf937a8153743a79b1613b4076a77769a19be6/src/flat.rs#L24