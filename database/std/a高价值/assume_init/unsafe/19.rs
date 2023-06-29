#![allow(unused)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

fn main() {
    // 引入 std::rc::Rc
    use std::rc::Rc;
    
    // 创建一个未初始化的 Rc 智能指针，指向 u32 类型的数据
    let mut five = Rc::<u32>::new_uninit();
    
    // 获取 Rc 智能指针的可变引用并写入 5
    Rc::get_mut(&mut five).unwrap().write(5);
    
    // 通过 unsafe 代码块，使用 assume_init() 方法将 Rc 智能指针转化为初始化后的 Rc<u32> 智能指针
    let five = unsafe { five.assume_init() };
    
    // 断言 Rc<u32> 中存储的值为 5
    assert_eq!(*five, 5)
}
