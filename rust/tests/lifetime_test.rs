/*
    生命周期的使用
        * 编译器需要保证引用的声明周期，不超过对象的存活时间
        * 有显示生命周期和隐式生命周期
        * 静态引用生命周期则说明该引用全程序有效
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

#[test]
fn test_static(){
    let static_ref : &'static str;
    //let static_ref : &str;
    static_ref = "hello";

    println!("{}", static_ref)
}