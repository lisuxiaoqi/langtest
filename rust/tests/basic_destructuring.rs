/*
Destructuring syntax（解构语法） = 用 模式匹配 的方式，把一个复合数据结构里的内容“拆开”绑定到变量。
它的核心就是 模式（pattern），用在：
let 绑定
match 表达式
if let / while let
函数参数
*/
// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

//tuple解构
#[test]
fn test_tuple_destructuring() {
    let tup = (10, "hello", true);
    let (a, b, c) = tup;
    println!("a = {}, b = {}, c = {}", a, b, c);
}

//struct解构
#[test]
fn test_struct_destructuring() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };

    //destruct数据到x, y
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    //destruct数据到x, y对应的别名
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);
}
//array解构
#[test]
fn test_array_destructuring() {
    let arr = [1, 2, 3];
    let [a, b, c] = arr;
    println!("a = {}, b = {}, c = {}", a, b, c);
}

//enum解构
#[test]
fn test_enum_destructuring() {
    enum Shape {
        Circle(i32),
        Rectangle { w: i32, h: i32 },
    }
    let shape = Shape::Circle(50);
    let rect = Shape::Rectangle { w: 50, h: 50 };

    if let Shape::Circle(size) = shape {
        println!("size:{}", size);
    }
    if let Shape::Rectangle { w, h } = rect {
        println!("w:{}, h:{}", w, h);
    }
}


//带引用解构
#[test]
fn test_ref_destructuring() {
    let tup = &(10, "hello".to_string(), true);
    let &(a, ref b, c) = tup;
    println!("a = {}, b = {}, c = {}", a, b, c);
}