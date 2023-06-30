impl Monitor for PythonMonitor {
    fn client_stats_mut(&mut self) -> &mut Vec<ClientStats> {
        let ptr = unwrap_me_mut!(self.wrapper, m, {
            m.client_stats_mut() as *mut Vec<ClientStats>
        });
        unsafe { ptr.as_mut().unwrap() }
    }
}
 
// https://github.com/AFLplusplus/LibAFL/blob/71aa0221a01482097715e695de0fe6197dc90aed/libafl/src/monitors/mod.rs#L1089