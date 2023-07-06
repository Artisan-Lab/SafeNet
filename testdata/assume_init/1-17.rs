pub fn now() -> SystemTime {
    let rtc = unsafe {
        let mut out = MaybeUninit::zeroed();
        expect_success(abi::SOLID_RTC_ReadTime(out.as_mut_ptr()), &"SOLID_RTC_ReadTime");
        out.assume_init()
    };
    let t = unsafe {
        libc::mktime(&mut libc::tm {
            tm_sec: rtc.tm_sec,
            tm_min: rtc.tm_min,
            tm_hour: rtc.tm_hour,
            tm_mday: rtc.tm_mday,
            tm_mon: rtc.tm_mon - 1,
            tm_year: rtc.tm_year,
            tm_wday: rtc.tm_wday,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: crate::ptr::null_mut(),
        })
    };
    assert_ne!(t, -1, "mktime failed");
    SystemTime(t)
}