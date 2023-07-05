fn read(&self) -> T {
    unsafe {
        let val = core::ptr::read_volatile(&self.0 as *const _);
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        core::arch::asm!("fence i,r");
        val
    }
}