fn main(){
    struct MyStruct<T> {
        data: Vec<T>,
    impl<T> MyStruct<T> {
        pub fn index(&self, index: usize) -> &T {
            unsafe {
                self.data.get_unchecked(index)
            }
        }
    }
    }
}
