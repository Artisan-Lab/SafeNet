async fn work(
    &mut self,
    _io: &mut WorkIo,
    sio: &mut StreamIo,
    _mio: &mut MessageIo<Self>,
    _meta: &mut BlockMeta,
) -> Result<()> {
    let o = sio.output(0).slice_unchecked::<u8>();

    unsafe {
        ptr::write_bytes(o.as_mut_ptr(), 0, o.len());
    }

    let before = self.n_produced / self.probe_granularity;
    let n = o.len() / std::mem::size_of::<T>();
    sio.output(0).produce(n);
    self.n_produced += n as u64;
    let after = self.n_produced / self.probe_granularity;

    for i in 1..=(after - before) {
        tracepoints::futuresdr::tx(self.id.unwrap(), before + i);
    }

    Ok(())
}
// https://github.com/FutureSDR/FutureSDR/blob/f3d9c3611885a5f5a7dfd9ab4455d1684fc03804/src/blocks/lttng/null_source.rs#L66