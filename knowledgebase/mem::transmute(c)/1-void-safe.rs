use std::mem;

struct Foo<T>{ v:T }
impl<T> Foo<T> {
    pub fn getptr(x: &mut Box<Self>) -> *mut libc::c_void {
        let ptr: *mut Self = x.as_mut();
        ptr as *mut libc::c_void
    }
}

fn main(){
    let x = &mut Box::new(Foo{v:1});
    let p = Foo::getptr(x);
    println!("{}", unsafe{(*x).v});
}
