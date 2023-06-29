use std::rc::Rc;
pub fn insert(&mut self, elem: T) -> bool {
    let chunk_index = chunk_index(elem);
    let chunk = &mut self.chunks[chunk_index];
    match *chunk {
        Zeros(chunk_domain_size) => {
            if chunk_domain_size > 1 {
                let words = Rc::<[Word; CHUNK_WORDS]>::new_zeroed();
                let mut words = unsafe { words.assume_init() };
                let words_ref = Rc::get_mut(&mut words).unwrap();
                words_ref[word_index] |= mask;
                *chunk = Mixed(chunk_domain_size, 1, words);
            } else {
                *chunk = Ones(chunk_domain_size);
            }
            true
        }
        Ones(_) => false,
    }
}
/*
https://github.com/esp-rs/rust/blob/f112def2207779c024b9ad42099077bb5c4c8998/compiler/rustc_index/src/bit_set.rs#L506
*/
