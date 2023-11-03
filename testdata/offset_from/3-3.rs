fn parse_entry_at_offset(data: &[u8], offset: u32) -> Result<Entry<'_>> {
    fn entry_impl(data: &[u8], offset: u32) -> Option<Result<Entry<'_>>> {
        let mut data = data.get(offset as usize..)?;
        let start = data.as_ptr();

        let lfh = data.read_pod::<LocalFileHeader>()?;
        if lfh.magic != LOCAL_FILE_HEADER_MAGIC {
            return Some(Err(Error::new(
                ErrorKind::InvalidData,
                "local file header contains invalid magic number",
            )))
        }

        if (lfh.flags & FLAG_ENCRYPTED) != 0 || (lfh.flags & FLAG_HAS_DATA_DESCRIPTOR) != 0 {
            return Some(Err(Error::new(
                ErrorKind::InvalidData,
                "attempted lookup of unsupported entry",
            )))
        }

        let path = data.read_slice(lfh.file_name_length.into())?;
        let path = Path::new(OsStr::from_bytes(path));

        let _extra = data.read_slice(lfh.extra_field_length.into())?;
        // SAFETY: Both pointers point into the same underlying byte array.
        let data_offset = offset as usize
            + usize::try_from(unsafe { data.as_ptr().offset_from(start) }).unwrap();
        let data = data.read_slice(lfh.compressed_size as usize)?;

        let entry = Entry {
            compression: lfh.compression,
            path,
            data_offset,
            data,
        };

        Some(Ok(entry))
    }

    entry_impl(data, offset).unwrap_or_else(|| {
        Err(Error::new(
            ErrorKind::InvalidData,
            "failed to read archive entry",
        ))
    })
}
//https://github.com/libbpf/blazesym/blob/e07c5adc3af1724e3ed89760141003e702984c06/src/zip.rs#L188