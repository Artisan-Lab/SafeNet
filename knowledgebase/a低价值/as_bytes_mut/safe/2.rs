fn main(){
    let mut s = String::from("🗻∈🌏");
// 将字符串转换为可变字符串
    let mut chars = s.chars().collect::<Vec<_>>();
// 替换第一个字符
    chars[0] = '🍔';
// 将字符数组转换为字符串
    s = chars.into_iter().collect();
    assert_eq!("🍔∈🌏", s);
}


/*
这段代码的目的是将一个 Unicode 字符串 s 中的第一个字符（🗻）替换为另一个 Unicode 字符（🍔）。
由于 Unicode 字符串是不可变的，所以需要使用 unsafe 关键字来直接访问 String 类型变量的底层字节数组并进行修改。
如果使用 safe 的方式来进行操作，需要先将字符串 s 转换为可变字符串，然后再替换第一个字符，最后再将字符串转换为不可变的。
*/
