fn downgrade_strong_executor(&mut self) {
    // SAFETY: 只会在一个 core 上运行，不需要考虑同步问题
    let mut old = self.strong_executor.clone();
    unsafe {
        Arc::get_mut_unchecked(&mut old).mark_weak();
    }
    self.add_weak_executor(old);
    self.strong_executor = Arc::new(Executor::new(self.task_collection.clone()));
}
/*
https://github.com/DeathWish5/PreemptiveScheduler/blob/e8cd353100046d2381f752261c3747bd9ee7b79e/src/runtime.rs#L70
*/