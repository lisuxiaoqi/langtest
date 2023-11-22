/*
    变量的声明定义：
        1. 不可变变量声明
        2. 可变变量声明
        3. 先声明，后绑定
*/

#[test]
fn test_declare() {
    //声明并赋值可变变量
    let a = 3;
    //声明并赋值不可变变量
    let mut b = 4;
    b = b + 1;
    println!("a:{}, b:{}", a, b);

    // 声明一个变量
    let a_binding;
    // 赋值前面声明的对象
    a_binding = 4;
    println!("a binding: {}", a_binding);
}
