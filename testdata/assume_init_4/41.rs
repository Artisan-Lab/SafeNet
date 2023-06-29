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
                let (word_index, mask) = chunk_word_index_and_mask(elem);
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
https://github.com/Artisan-Lab/Rust-SGNN/blob/main/data/assume_init/4-rc-slice-unsafe.rs
*/