fn main() {
    let mut arr = [0; 1000000];

    // 通过 get_unchecked() 方法访问和修改数组元素
    unsafe {
        for i in (0..arr.len()).step_by(2) {
            let val = arr.get_unchecked_mut(i);
            *val = i as i32;
        }
    }

    // 计算数组中大于 100 的元素个数
    let mut count = 0;
    for i in 0..arr.len() {
        unsafe {
            let val = arr.get_unchecked(i);
            if *val > 100 {
                count += 1;
            }
        }
    }
    println!("There are {} elements greater than 100", count);
}

