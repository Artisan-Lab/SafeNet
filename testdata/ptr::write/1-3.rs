fn apply_on_addr(&self, addr: *mut u8) {
    match self {
        AddrOp::Read => {
            // We have to do something perform a read_volatile, otherwise
            // the Release version will optimize it out, making the test fail.
            unsafe { std::ptr::read_volatile(addr) };
        }
        AddrOp::Write => unsafe {
            std::ptr::write(addr, 0xFF);
        },
    }
}
// https://github.com/firecracker-microvm/firecracker/blob/cd2eddeb50b2a702cce60a8116fc8617eee206ac/src/utils/src/vm_memory.rs#L471