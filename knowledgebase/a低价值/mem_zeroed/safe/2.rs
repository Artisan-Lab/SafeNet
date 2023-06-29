struct MyStruct {
    x: i32,
    y: bool,
    z: [u8; 16],
}

fn main() {
    let s = MyStruct {
        x: 0,
        y: false,
        z: [0; 16],
    };
}