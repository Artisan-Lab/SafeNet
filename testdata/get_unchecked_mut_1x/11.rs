fn main() {
    struct Slab<T> {
        data: Vec<T>,
        free: VecDeque<usize>,
    }

    impl<T> Slab<T> {
        unsafe fn get_unchecked_mut(&mut self, id: usize) -> &mut T {
            self.data.get_unchecked_mut(id)
        }
    }
}

/*
https://github.com/iacore/dioxus/blob/d521da1991719760e271457dfe4f9ddf281afbb3/packages/native-core/src/tree.rs#L288
 */