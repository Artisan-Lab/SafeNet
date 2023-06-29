pub fn multilook_and_floor(x:Arc<TwoDArray>, row_factor:usize, col_factor:usize, num_cores:usize) -> TwoDArray {

    let newrow = x.rows/row_factor;
    let newcol = x.cols/col_factor;
    let output:Arc<TwoDArray> = Arc::new(TwoDArray::zeros(newrow, newcol));


    let subsum = move |r:&Arc<TwoDArray>,i:usize,j:usize| -> f64 {
	let mut total = 0.0;
        let a_end = row_factor;
        let b_end = col_factor;
        let norm = 1.0/((a_end*b_end) as f64);
        for a in 0..a_end {
            for b in 0..b_end {
                total += r[(row_factor*i+a,col_factor*j+b)] * norm;
            }
        }
	return total;
    };
    
    // Since the array is row major, we divide jobs by number of rows
    let mut thread_handles:Vec<_> = (0..num_cores).map( {
	|t| {
	    let start_dest_row = t*newrow/num_cores;
	    let end_dest_row = if t < num_cores -1 {(t+1)*newrow/num_cores} else {newrow};
	    let x_ = x.clone();
	    let mut o_ = output.clone();
	    thread::spawn(move ||{
		let op = unsafe{Arc::get_mut_unchecked(&mut o_)};
		for i in start_dest_row..end_dest_row {
		    for j in 0..newcol {
			op[(i,j)] = subsum(&x_, i, j).max(0.0).sqrt();
		    }
		}
	    })
	}
    }).collect();

    for _i in 0..num_cores {thread_handles.pop().unwrap().join().unwrap();}

    Arc::try_unwrap(output).unwrap()
}
/*
https://github.com/PeterQLee/sentinel1_denoise_rs/blob/c95b2cd69669ba5f8025cdf6e71939c26bbabfb5/engine/src/postprocess.rs#L66
*/
