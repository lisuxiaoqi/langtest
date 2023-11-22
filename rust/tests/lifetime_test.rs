/*
    生命周期的使用
*/
struct MyStruct<'a> {
    data: &'a str,
}

#[test]
fn test_lifetime() {
    let string_data = String::from("Hello, Rust!");
    let my_struct_instance;

    {
        let reference = &string_data;
        my_struct_instance = MyStruct { data: reference };
    }

    // 在这里，string_data 已经离开作用域，但是 my_struct_instance 中仍然包含对 string_data 的引用。
    // 由于生命周期参数 'a，Rust 确保在引用的生命周期内访问引用的数据是安全的。
    println!("{}", my_struct_instance.data);
}
