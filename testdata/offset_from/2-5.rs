fn consume_read_buf(
    rb0: &mut BytesMut,
    rb: &mut &[u8],
    ptr0: *const u8,
    len0: usize,
) -> BaseRtResult<()> {
    //consume rb0
    let cnt = unsafe { (*rb).as_ptr().offset_from(ptr0) };
    //if cnt throws err
    rb0.advance(cnt as usize);
    if len0 > cnt as usize {
        log::debug!(
            "only partially consume {} in all {} bytes this time",
            cnt,
            len0
        );
        // Ok(false) //has other kinds of packets, to drain
    }
    Ok(())
}
//https://github.com/tensorbase/tensorbase/blob/7b071a88a175da64e5c8cca7f1710940fe9b75ab/crates/runtime/src/ch/messages.rs#L210