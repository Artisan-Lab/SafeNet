fn dump_kv_stats(&self) -> Result<String> {
    let mut kv_str = box_try!(MiscExt::dump_stats(&self.engines.kv));
    if let Some(s) = self.kv_statistics.as_ref() && let Some(s) = s.to_string() {
        kv_str.push_str(&s);
    }
    Ok(kv_str)
}
/*
https://github.com/pingcap/tidb-engine-ext/blob/023c4e72619333356ba7d808361c2360768dcd77/src/server/debug.rs#L921
*/