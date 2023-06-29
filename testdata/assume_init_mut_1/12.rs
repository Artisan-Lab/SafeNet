pub fn finalize(&mut self)
    {
        let mut counts_str = String::new();
        for c in &self.counts {
            counts_str.push_str(&format!("{} ",c));
        }
        let stats_line = format!("{}: {}",self.q_id,counts_str);
        unsafe
        {
        write!(STATS_FILE.assume_init_mut().lock().unwrap(), "{}\n", stats_line).expect("Error writing stats line.");
        }
    }
    // https://github.com/ekimb/rust-mdbg/blob/985ba674f9da0ff440b0330fd1e2b1de92226305/src/read_stats.rs#L62