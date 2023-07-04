/*
https://github.com/sharkdp/hyperfine/blob/1098ab6659d29b9289d8d0dd0ba971d1bbb6c132/src/timer/unix_timer.rs#L40
*/

fn get_cpu_times() -> CPUTimes {
    use libc::{getrusage, rusage, RUSAGE_CHILDREN};

    let result: rusage = unsafe {
        let mut buf = mem::zeroed();
        let success = getrusage(RUSAGE_CHILDREN, &mut buf);
        assert_eq!(0, success);
        buf
    };

    const MICROSEC_PER_SEC: i64 = 1000 * 1000;

    #[allow(clippy::useless_conversion)]
    CPUTimes {
        user_usec: i64::from(result.ru_utime.tv_sec) * MICROSEC_PER_SEC
            + i64::from(result.ru_utime.tv_usec),
        system_usec: i64::from(result.ru_stime.tv_sec) * MICROSEC_PER_SEC
            + i64::from(result.ru_stime.tv_usec),
    }
}