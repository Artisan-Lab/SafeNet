pub fn benchmark(bench: BenchmarkOptions) -> Result<BenchmarkResults, String> {

    let mut f = File::open(bench.input_path).unwrap();//bad
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();//bad


    let len = buffer.len();

    let mem = buffer.as_mut_ptr();

    let cap = buffer.capacity();


    mem::forget(buffer);

    let pool = threadpool::ThreadPool::new(bench.thread_count);

    let (tx, rx) = channel();



    let begin_at = time::precise_time_ns();

    for _ in 0..bench.run_count {
        let tx = tx.clone();
        let m = mem.clone() as i64;
        let l = len.clone();
        let cmds = bench.commands.clone();
        pool.execute(move || {
            tx.send(benchmark_op(cmds, m as *mut u8, l)).unwrap();
        });
    }

    let mut res_list = Vec::new();
    let result_iterator = rx.iter().take(bench.run_count as usize);
    for i in result_iterator {
        match i.result {
            Ok(_) => {
                res_list.push(i);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    let end_at = time::precise_time_ns();

    unsafe {
        let _ = Vec::from_raw_parts(mem, len, cap);
    }

    return Ok(BenchmarkResults {
        list: res_list,
        wall_nanoseconds: (end_at - begin_at) as i64,
        threads: bench.thread_count,
        count: bench.run_count,
    });
}
/*
https://github.com/Geal/imageflow/blob/365b2f1b66b871918f39df77a0de124cd0238695/imageflow_core/src/boring.rs#L278
*/