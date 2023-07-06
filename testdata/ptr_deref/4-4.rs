fn test_user_data() {
    let user_data = Box::new(10u8);
    let mut inner: io_uring_sqe = unsafe { std::mem::zeroed() };
    inner.user_data = Box::into_raw(user_data) as u64;

    let sqe: Sqe = Sqe::new(inner);

    assert_eq!(unsafe { sqe.user_data::<u8>() }, 10);
}
//https://github.com/firecracker-microvm/firecracker/blob/a9300c13c823e4192a042a2d0c765e0bae5a6319/src/vmm/src/io_uring/operation/sqe.rs#L45