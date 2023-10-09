extern "C" fn initialize_decoder(buffer_ptr: *const u8, len: i32, codec: i32) {
    let codec = if codec == 0 {
        CodecType::H264
    } else {
        CodecType::Hevc
    };

    let mut config_buffer = vec![0; len as usize];

    unsafe { ptr::copy_nonoverlapping(buffer_ptr, config_buffer.as_mut_ptr(), len as usize) };

    if let Some(sender) = &*VIDEO_MIRROR_SENDER.lock() {
        sender.send(config_buffer.clone()).ok();
    }

    if let Some(file) = &mut *VIDEO_RECORDING_FILE.lock() {
        file.write_all(&config_buffer).ok();
    }

    *DECODER_CONFIG.lock() = Some(DecoderInitializationConfig {
        codec,
        config_buffer,
    });
}
// https://github.com/alvr-org/ALVR/blob/00a19bf54f283fc8aaf816b7cb4b09c3a99369d2/alvr/server/src/lib.rs#L328