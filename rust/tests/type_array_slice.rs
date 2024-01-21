/*
    测试Array和slice基本用法
        数组标注为[T; length]
        slice标注为&[T]
*/
use std::mem;
//slice 的类型标记为 &[T]
fn analysize_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

//测试数组的基本使用
#[test]
fn test_array_slice() {
    //数组通过 [T; length]的方式标注
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素可以初始化成相同的值
    let _ys: [i32; 5] = [0; 5];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    //通过引用&，把Array转化为slice
    analysize_slice(&xs);

    //可以把Array的一部分转化为slice
    analysize_slice(&xs[1..3]);
}
