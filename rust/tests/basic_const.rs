/*
   常量的声明和使用测试
   有两种常量，可以在任意作用域声明，包括全局作用域。它们都需要显式的类型声明：
        const：不可改变的值（通常使用这种）。
        static：具有 'static 生命周期的，可以是可变的变量（须使用 static mut 关键字）
*/

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

#[test]
fn test_const() {
    fn is_big(n: i32) -> bool {
        // 在一般函数中访问常量
        n > THRESHOLD
    }

    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    //THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}

enum E {
    First = 0,
    Second = 2,
}
const FLAG: bool = (E::First as usize) == 0 || (E::Second as usize) == 0;
#[test]
fn test_const_bool() {
    eprintln!("flag:{}", FLAG);
}
