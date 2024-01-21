/*
    turbofish语法是指在调用函数时，显式指定泛型参数的语法
        foo::<paramType>(param)
*/

#[test]
fn test() {
    //定义一个使用泛型的函数
    fn pair<T, U>(first: T, second: U) -> (T, U) {
        (first, second)
    }

    //不使用turbofish，系统自动推断类型
    let p = pair(13, 14);
    println!("{:?}", p);

    //使用turbofish语法显式指定泛型类型
    let sp = pair::<u32, u32>(13, 14);
    println!("turbofish: {:?}", sp);
}
