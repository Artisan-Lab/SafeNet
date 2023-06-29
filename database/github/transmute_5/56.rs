pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    let pid = std::process::id();
    let mut file = File::open(format!("/proc/{}/status", pid)).ok()?;
    let mut buffer: [u8; 16] = [0; 16];
    // Read 4 bytes after initial 12 bytes and convert into 32-byte uint.
    file.read_exact(&mut buffer).ok()?;
    let nlwp_bytes = <[u8; 4]>::try_from(&buffer[12..16]).ok()?;
    let nlwp = unsafe { std::mem::transmute::<[u8; 4], u32>(nlwp_bytes) };
    NonZeroUsize::new(nlwp as usize)
}
