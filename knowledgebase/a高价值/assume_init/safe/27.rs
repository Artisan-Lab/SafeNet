use std::sync::Arc;
fn main(){
    let five = Arc::new(5);
    assert_eq!(*five, 5);
}