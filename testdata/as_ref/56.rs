fn dump_raft_stats(&self) -> Result<String> {
    let mut raft_str = box_try!(RaftEngine::dump_stats(&self.raft_engine));
    if let Some(s) = self.raft_statistics.as_ref() && let Some(s) = s.to_string() {
        raft_str.push_str(&s);
    }
    Ok(raft_str)
}
/*
https://github.com/tikv/tikv/blob/92a77ac3547f4ab9ffd8d51f150797704e27a320/src/server/debug2.rs#L784
*/