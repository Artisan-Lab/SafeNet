/*
https://github.com/SpaceBlocks/deno/blob/13e3acf71dd8443d8d158087d8c0a0cb72167f9a/cli/js.rs#L20
*/

pub static CLI_SNAPSHOT: Lazy<Box<[u8]>> = Lazy::new(
  #[allow(clippy::uninit_vec)]
  #[cold]
  #[inline(never)]
  || {
    static COMPRESSED_CLI_SNAPSHOT: &[u8] =
      include_bytes!(concat!(env!("OUT_DIR"), "/CLI_SNAPSHOT.bin"));

    let size =
      u32::from_le_bytes(COMPRESSED_CLI_SNAPSHOT[0..4].try_into().unwrap())
        as usize;
    let mut vec = Vec::with_capacity(size);

    // SAFETY: vec is allocated with exact snapshot size (+ alignment)
    // SAFETY: non zeroed bytes are overwritten with decompressed snapshot
    unsafe {
      vec.set_len(size);
    }

    lzzzz::lz4::decompress(&COMPRESSED_CLI_SNAPSHOT[4..], &mut vec).unwrap();

    vec.into_boxed_slice()
  },
);