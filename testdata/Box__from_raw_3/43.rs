pub fn cleanup() {
    let hbs_opt_box: Option<Box<HashMap<ProfilerCategory, Heartbeat>>> = lock_and_work(|hbs_opt|
        hbs_opt.take().map(|hbs_ptr|
            unsafe {
                Box::from_raw(hbs_ptr)
            }
        )
    );
    if let Some(mut hbs) = hbs_opt_box {
        for (_, v) in hbs.iter_mut() {
            log_heartbeat_records(v);
        }
        hbs.clear();
    }
}
/*
https://github.com/karldoenitz/servo/blob/d795ceae17b2ce5346d4f211f73c62e5c25dce79/components/profile/heartbeats.rs#L31
*/