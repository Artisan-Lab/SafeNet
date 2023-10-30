fn raw_encode<'a>(
    &self,
    buf: &'a mut [u8],
    purpose: &Option<RawEncodePurpose>,
) -> Result<&'a mut [u8], BuckyError> {
    let bytes = self.raw_measure(purpose).unwrap();
    if buf.len() < bytes {
        let msg = format!(
            "not enough buffer for encode Signature buf, except={}, got={}",
            bytes,
            buf.len()
        );
        error!("{}", msg);

        return Err(BuckyError::new(BuckyErrorCode::OutOfLimit, msg));
    }

    // sign_source_with_ref_index
    let buf = self.sign_source_with_ref_index().raw_encode(buf, purpose)?;

    // signatory: Option<SignatureSource>
    let buf = match &self.sign_source {
        SignatureSource::RefIndex(t) => {
            let buf = t.raw_encode(buf, purpose)?;
            buf
        }
        SignatureSource::Object(t) => {
            let buf = t.raw_encode(buf, purpose)?;
            buf
        }
        SignatureSource::Key(t) => {
            let buf = t.raw_encode(buf, purpose)?;
            buf
        }
    };

    // sign_time
    let buf = self.sign_time.raw_encode(buf, purpose)?;

    // sign_data: Vec<u8>
    let buf = match self.sign {
        SignData::Rsa1024(sign) => {
            let buf = KEY_TYPE_RSA.raw_encode(buf, purpose)?;
            let bytes = std::mem::size_of::<u32>() * U32::to_usize();
            unsafe {
                std::ptr::copy(
                    sign.as_slice().as_ptr() as *const u8,
                    buf.as_mut_ptr(),
                    bytes,
                );
            }
            &mut buf[bytes..]
        }
        SignData::Rsa2048(sign) => {
            let buf = KEY_TYPE_RSA2048.raw_encode(buf, purpose)?;
            let bytes = std::mem::size_of::<u32>() * U64::to_usize();
            unsafe {
                std::ptr::copy(
                    sign.as_slice().as_ptr() as *const u8,
                    buf.as_mut_ptr(),
                    bytes,
                );
            }
            &mut buf[bytes..]
        }
        SignData::Rsa3072(sign) => {
            let buf = KEY_TYPE_RSA3072.raw_encode(buf, purpose)?;
            let bytes = std::mem::size_of::<u32>() * U96::to_usize();
            unsafe {
                std::ptr::copy(
                    sign.as_slice().as_ptr() as *const u8,
                    buf.as_mut_ptr(),
                    bytes,
                );
            }
            &mut buf[bytes..]
        }
        SignData::Ecc(sign) => {
            let buf = KEY_TYPE_SECP256K1.raw_encode(buf, purpose)?;
            let bytes = std::mem::size_of::<u32>() * U16::to_usize();
            unsafe {
                std::ptr::copy(
                    sign.as_slice().as_ptr() as *const u8,
                    buf.as_mut_ptr(),
                    bytes,
                );
            }
            &mut buf[bytes..]
        }
    };

    Ok(buf)
}
// https://github.com/buckyos/CYFS/blob/e030188895096fd8d91d48753877729f4d37dd24/src/component/cyfs-base/src/crypto/signature.rs#L251