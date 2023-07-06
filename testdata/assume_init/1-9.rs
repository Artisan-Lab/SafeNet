fn default() -> Packet {
    Packet {
        buffer: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        meta: Meta::default(),
    }
}

// https://github.com/tribe-health/solana/blob/032bee13abd12ab2882d475c78a4b3d9e2d5f7bd/sdk/src/packet.rs#L134