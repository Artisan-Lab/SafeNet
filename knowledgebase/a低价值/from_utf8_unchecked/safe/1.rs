// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// 将字节序列转换为 UTF-8 编码的字符串
let sparkle_heart = match str::from_utf8(&sparkle_heart) {
    Ok(s) => s,
    Err(e) => {
        eprintln!("Error: {}", e);
        // 处理解码失败的情况
        ""
    }
};

assert_eq!("💖", sparkle_heart);
