pub fn parse_timestamp_from_string<T>(s: T, fmt: &str) -> Result<(usize, UnixTimestamp)>
    where
        T: Into<Vec<u8>>,
{
    let mut new_tm: libc::tm = unsafe { std::mem::zeroed() };
    let fmt: Cow<'_, CStr> = if let Ok(cs) = CStr::from_bytes_with_nul(fmt.as_bytes()) {
        Cow::from(cs)
    } else {
        Cow::from(CString::new(fmt.as_bytes())?)
    };
    unsafe {
        let val = CString::new(s)?;
        let ret = strptime(val.as_ptr(), fmt.as_ptr(), std::ptr::addr_of_mut!(new_tm));
        if ret.is_null() {
            return Err("Could not parse time with strptime.".into());
        }
        let rest: isize = val.as_ptr().offset_from(ret);
        Ok((
            rest.unsigned_abs(),
            mktime(std::ptr::addr_of!(new_tm)) as u64,
        ))
    }
}
//https://github.com/meli/meli/blob/7998e1e77ef057bab28434edefb79d7be6a4de33/melib/src/utils/datetime.rs#L511