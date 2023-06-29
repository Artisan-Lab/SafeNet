pub fn switch() {
    let interrupt_enabled = is_interrupt_enabled();

    let prev_proc = current_process_arc().clone();
    let (prev_pid, prev_state) = {
        let prev = prev_proc.lock();
        (prev.pid(), prev.state)
    };
    let next_proc = {
        let scheduler = SCHEDULER.lock();
        if prev_pid != PId::new(0) && prev_state == ProcessState::Runnable {
            scheduler.enqueue(prev_pid);
        }
        match scheduler.pick_next() {
            Some(next_pid) => PROCESSES.lock().get(&next_pid).unwrap().clone(),
            None => IDLE_THREAD.get().get().clone(),
        }
    };

    if Arc::ptr_eq(&prev_proc, &next_proc) {
        return;
    }

    let mut prev = prev_proc.lock();
    let mut next = next_proc.lock();
    debug_assert!(next.state() == ProcessState::Runnable);

    debug_assert!(HELD_LOCKS.get().is_empty());
    HELD_LOCKS.as_mut().push(prev_proc.clone());
    HELD_LOCKS.as_mut().push(next_proc.clone());

    if let Some(vm) = next.vm.as_ref() {
        let lock = vm.lock();
        lock.page_table().switch();
    }
    debug_assert!(Arc::strong_count(&next_proc) > 1);
    unsafe {
        Arc::decrement_strong_count(Arc::as_ptr(&prev_proc));
        Arc::decrement_strong_count(Arc::as_ptr(&next_proc));
    }

    CURRENT.as_mut().set(next_proc.clone());
    arch::switch_thread(&mut prev.arch, &mut next.arch);
    mem::forget(prev);
    mem::forget(next);


    mem::forget(prev_proc);
    mem::forget(next_proc);

    after_switch();
    if interrupt_enabled {
        unsafe {
            enable_interrupt();
        }
    }
}
/*
https://github.com/nuta/archives/blob/3db15f62dc1b4497a8c9a39e32f997470653157f/kerla-demo/kerla/kernel/process/switch.rs#L67
*/