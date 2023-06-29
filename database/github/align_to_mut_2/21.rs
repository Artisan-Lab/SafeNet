pub fn open(mut file: F) -> Result<Self, OpenError> {
    // Seek to beginning.
    if let Err(e) = file.rewind() {
        return Err(OpenError::SeekFailed(0, e));
    }

    // Check header.
    let mut hdr: [u8; 48] = [0u8; 48];

    if let Err(e) = file.read_exact(&mut hdr) {
        return Err(if e.kind() == ErrorKind::UnexpectedEof {
            OpenError::TooSmall
        } else {
            OpenError::IoFailed(e)
        });
    }

    let magic = &hdr[0..4];

    if &magic != b"PFSC" {
        return Err(OpenError::InvalidMagic);
    }

    // Read header.
    let block_size = LE::read_u32(&hdr[0x0c..]); // BlockSz
    let original_block_size = LE::read_u64(&hdr[0x10..]); // BlockSz2
    let block_offsets = LE::read_u64(&hdr[0x18..]); // BlockOffsets
    let original_size = LE::read_u64(&hdr[0x28..]); // DataLength

    // Read block offsets.
    if let Err(e) = file.seek(SeekFrom::Start(block_offsets)) {
        return Err(OpenError::SeekFailed(block_offsets, e));
    }

    let original_block_count = original_size / original_block_size + 1;
    let mut compressed_blocks: Vec<u64> = vec![0; original_block_count as usize];
    let buf = unsafe {
        let (pre, mid, pos) = compressed_blocks.align_to_mut::<u8>();

        assert!(pre.is_empty());
        assert!(pos.is_empty());

        mid
    };

    if let Err(e) = file.read_exact(buf) {
        return Err(OpenError::ReadBlockMappingFailed(e));
    }

    Ok(Self {
        file,
        block_size,
        original_block_size,
        compressed_blocks,
        original_size,
        current_offset: 0,
        current_block: Vec::new(),
    })
}

// https://github.com/obhq/obliteration/blob/9da8e5e2795e2fa28952961b30bdc5d2e2b1596e/src/pfs/src/pfsc.rs#L55