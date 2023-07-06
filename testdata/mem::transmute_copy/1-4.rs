fn get_pair_mut(&mut self, a: usize, b: usize, same_index_error_message: &'static str)
                    -> (&mut T, &mut T) {
        if a == b {
            panic!(same_index_error_message)
        }
        unsafe {
            let self2 = mem::transmute_copy::<&mut Vec<T>, &mut Vec<T>>(&self);
            (&mut self[a], &mut self2[b])
        }
    }
// https://github.com/SimonSapin/rust-forest/blob/1d18375b42d1fb92cba9bd0a6d52713df7a10601/idtree/lib.rs#L75