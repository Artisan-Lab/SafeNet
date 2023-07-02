pub fn decompress<T: Read + Seek>(data: &mut T) -> Result<Vec<u8>, Error> {
    let lz_type = data.read_u8()?;
    let mut data_size_tmp: [u8; 4] = [0; 4];
    data.read_exact(&mut data_size_tmp[0..3])?;
    let data_size = unsafe { transmute::<[u8; 4], u32>(data_size_tmp) };

    match lz_type {
        0x10 => decompress_lzss10(data, data_size as usize),
        0x11 => decompress_lzss11(data, data_size as usize),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid header")),
    }
}
