pub async fn clean_data_sources(&mut self) -> Result<()> {
    // Breadth-first search
    let mut queue = VecDeque::new();
    self.plan().await?.into_iter().for_each(|plan| {
        queue.push_back(plan);
    });

    while !queue.is_empty() {
        let mut plan = queue.pop_front().unwrap();
        if plan.children().is_empty() {
            let schema = plan.schema().clone();
            unsafe {
                Arc::get_mut_unchecked(&mut plan)
                    .as_mut_any()
                    .downcast_mut::<MemoryExec>()
                    .unwrap()
                    .set_partitions(vec![vec![RecordBatch::new_empty(schema)]]);
            }
        }

        plan.children()
            .iter()
            .enumerate()
            .for_each(|(i, _)| queue.push_back(plan.children()[i].clone()));
    }

    Ok(())
}
/*
https://github.com/flock-lab/flock/blob/44943dc3ae60707c824c59b787f128e51ed08fec/flock/src/runtime/context.rs#L237
*/
