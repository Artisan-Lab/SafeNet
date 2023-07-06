pub(crate) fn append_line_offsets(source: &str, out: &mut Vec<raw::LineOffset>) {
    // The empty file has only one line offset for the start.
    if source.is_empty() {
        out.push(raw::LineOffset(0));
        return;
    }

    let buf_ptr = source.as_ptr();
    out.extend(source.lines().map(move |line| {
        raw::LineOffset(unsafe { line.as_ptr().offset_from(buf_ptr) as usize } as u32)
    }));

    // If the file ends with a line break, add another line offset for the empty last line
    // (the lines iterator skips it).
    if source.ends_with('\n') {
        out.push(raw::LineOffset(source.len() as u32));
    }
}
//https://github.com/getsentry/symbolic/blob/09c291ba428303b76b0a8501d0b19ed9333abea0/symbolic-sourcemapcache/src/writer.rs#L310