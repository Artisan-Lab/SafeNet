pub fn initialize(
    self: &'static Arc<Self>,
    num_workers: usize,
    context: &'static C,
    tls: VMThread,
) {
    use crate::scheduler::work_bucket::WorkBucketStage::*;
    let num_workers = if cfg!(feature = "single_worker") {
        1
    } else {
        num_workers
    };

    let mut self_mut = self.clone();
    let self_mut = unsafe { Arc::get_mut_unchecked(&mut self_mut) };

    self_mut.context = Some(context);
    self_mut.coordinator_worker = Some(RwLock::new(Worker::new(
        0,
        Arc::downgrade(&self),
        true,
        self.channel.0.clone(),
    )));
    self_mut.worker_group = Some(WorkerGroup::new(
        num_workers,
        Arc::downgrade(&self),
        self.channel.0.clone(),
    ));
    self.worker_group
        .as_ref()
        .unwrap()
        .spawn_workers(tls, context);

    {
        // Unconstrained is always open. Prepare will be opened at the beginning of a GC.
        // This vec will grow for each stage we call with open_next()
        let mut open_stages: Vec<WorkBucketStage> = vec![Unconstrained, Prepare];
        // The rest will open after the previous stage is done.
        let mut open_next = |s: WorkBucketStage| {
            let cur_stages = open_stages.clone();
            self_mut.work_buckets[s].set_open_condition(move || {
                let should_open =
                    self.are_buckets_drained(&cur_stages) && self.worker_group().all_parked();
                // Additional check before the `RefClosure` bucket opens.
                if should_open && s == WorkBucketStage::RefClosure {
                    if let Some(closure_end) = self.closure_end.lock().unwrap().as_ref() {
                        if closure_end() {
                            // Don't open `RefClosure` if `closure_end` added more works to `Closure`.
                            return false;
                        }
                    }
                }
                should_open
            });
            open_stages.push(s);
        };

        open_next(Closure);
        open_next(RefClosure);
        open_next(RefForwarding);
        open_next(Release);
        open_next(Final);
    }
}
/*
https://github.com/osa1/mmtk-core/blob/17b2537e9a3bcc8ead34c4288d5b21efdca80404/src/scheduler/scheduler.rs#L100
*/