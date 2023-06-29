let v = vec![1, 2, 3];
let mut v = std::mem::ManuallyDrop::new(v);

let mut new_vec = Vec::with_capacity(v.len());
for i in 0..v.len() {
    new_vec.push(4 + i);
}

let rebuilt = std::mem::ManuallyDrop::new(new_vec);

// 继续使用 rebuilt 中的值，不需要解引用指针或进行指针运算

// rebuilt 手动释放内存
std::mem::ManuallyDrop::into_inner(rebuilt);

// v 手动释放内存
std::mem::ManuallyDrop::into_inner(v);
