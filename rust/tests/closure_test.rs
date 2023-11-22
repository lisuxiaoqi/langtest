/*
    闭包的使用
        * 闭包的定义
        * 闭包capture变量
        * Fn, FnOnce, FnMut
*/

//测试闭包函数的显示定义和隐式定义
//* 显式：指定输入输出类型
//* 隐式：不指定类型
#[test]
fn test_define() {
    //显式定义闭包函数
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("add result:{}", add(1, 3));

    //隐式定义闭包函数
    let add2 = |x, y| x + y;
    println!("add2 result:{:?}", add2(1, 3));

    //部分定义类型
    let add3 = |x: i32, y| -> i32 { x + y };
    println!("add3 result:{:?}", add3(1, 3));
}
