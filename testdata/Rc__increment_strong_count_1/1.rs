impl<'long: 'short, 'short> Drop for FS<'long, 'short> {
    fn drop(&mut self) {
        for (_, account_info) in self.allocators.values_mut() {
            let cell = account_info.data.clone();
            unsafe {
                let ptr = Rc::into_raw(cell);
                Rc::decrement_strong_count(ptr);

                let mut ref_counter = Rc::from_raw(ptr);
                Rc::get_mut(&mut ref_counter)
                    .expect("Already borrowed")
                    .undo_leak();

                Rc::increment_strong_count(ptr);
            }
        }
    }
}