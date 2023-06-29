pub fn create() -> (Arc<Self>, Arc<Self>) {
    let mut channel0 = Arc::new(Channel {
        base: KObjectBase::default(),
        peer: Weak::default(),
        recv_queue: Default::default(),
    });
    let channel1 = Arc::new(Channel {
        base: KObjectBase::default(),
        peer: Arc::downgrade(&channel0),
        recv_queue: Default::default(),
    });
    unsafe {
        Arc::get_mut_unchecked(&mut channel0).peer = Arc::downgrade(&channel1);
    }
    (channel0, channel1)
}

/*
https://github.com/rcore-os/zCore-Tutorial/blob/e41dd13f904db29b131c20afb1f1a838255bd509/docs/src/ch01-03-channel-object.md?plain=1#L117
 */