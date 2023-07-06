pub fn insert(&mut self, elem: T) -> bool {
    assert!(elem.index() < self.domain_size);
    let chunk_index = chunk_index(elem);
    let chunk = &mut self.chunks[chunk_index];
    match *chunk {
        Zeros(chunk_domain_size) => {
            if chunk_domain_size > 1 {
                // We take some effort to avoid copying the words.
                let words = Rc::<[Word; CHUNK_WORDS]>::new_zeroed();
                // SAFETY: `words` can safely be all zeroes.
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
        Mixed(chunk_domain_size, ref mut count, ref mut words) => {
            // We skip all the work if the bit is already set.
            let (word_index, mask) = chunk_word_index_and_mask(elem);
            if (words[word_index] & mask) == 0 {
                *count += 1;
                if *count < chunk_domain_size {
                    let words = Rc::make_mut(words);
                    words[word_index] |= mask;
                } else {
                    *chunk = Ones(chunk_domain_size);
                }
                true
            } else {
                false
            }
        }
    }
}