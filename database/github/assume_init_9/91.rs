pub struct Packet {
    buffer: [u8; PACKET_DATA_SIZE],
    pub meta: Meta,
}
impl Default for Packet {
    fn default() -> Packet {
        Packet {
            buffer: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            meta: Meta::default(),
        }
    }
}
/*
https://github.com/ChorusOne/solana-mev/blob/c60c05a7e620b95b6c240ea56cd0180fcb331c7d/sdk/src/packet.rs#L41
*/