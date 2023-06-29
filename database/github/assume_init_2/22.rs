fn resize_position_table(&mut self) {
    let old = std::mem::replace(
        &mut self.pos_table,
        PositionTable {
            size: new_size,
            shift: new_shift,
            mask: new_size - 1,
            threshold: threshold(new_size),
            present: unsafe { Box::new_zeroed_slice(bitvect_size(new_size)).assume_init() },
            entries: Box::new_uninit_slice(new_size),
        },
    );
}

/*
https://github.com/PinkDiamond1/hhvm/blob/9592c3ee4d91b5870587f45fbf481382ebab5d21/hphp/hack/src/ocamlrep_marshal/ser.rs#L240
*/