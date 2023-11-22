/*
    集成测试的hello world
*/
//代测试函数
fn add_two(a: i32) -> i32 {
    a + 2
}

//#[test]表示cargo test会执行该测试函数
#[test]
fn test_hello() {
    //调用代测试函数，使用assert系列
    assert_eq!(4, add_two(2));
    //可以输出到console，需要配合--show-output使用
    println!("Hello World")
}
