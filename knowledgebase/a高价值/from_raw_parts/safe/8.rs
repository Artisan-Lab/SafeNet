// 假设我们有一个不确定的字节序列
let bytes = vec![104, 101, 108, 108, 111, 226, 152, 131];

// 将字节序列尝试转换为 String 类型
let s = match String::from_utf8(bytes) {
    Ok(s) => s,
    Err(e) => {
        eprintln!("Error: {}", e);
        // 处理解码失败的情况
        String::new()
    }
};

println!("{}", s); 
