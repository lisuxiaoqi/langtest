/*
    let关键字可以用于模式匹配
*/

#[test]
fn test_let() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };
    //通过let关键字模式匹配来获取x,y值,并赋值给变量x_value, y_value
    let Point {
        x: x_value,
        y: y_value,
    } = p;

    println!("x_value:{}, y_value:{}", x_value, y_value);
}
