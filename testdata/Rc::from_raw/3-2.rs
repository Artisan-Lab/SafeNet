fn pull_out_packet(&mut self, _cb_queue: &mut CallbackQueue) -> Option<PacketRc> {
    let packet = Worker::with_active_host(|host| unsafe {
        c::legacysocket_pullOutPacket(self.as_legacy_socket(), host)
    })
    .unwrap();

    if packet.is_null() {
        return None;
    }

    Some(PacketRc::from_raw(packet))
}