/*
    测试where的用法:
        where用于描述约束trait，目的是使语法更为直观
        where大部分时候可以被T：Type方式替代，但有时候只能使用where
*/
use std::fmt::Debug;

// 不使用 where 的版本
fn compare_nowhere<T: std::cmp::PartialOrd>(a: T, b: T) -> bool {
    a > b
}

// 使用 where 的版本，功能和不使用where版本相同，仅语法格式不同
fn compare_where<T>(a: T, b: T) -> bool
where
    T: std::cmp::PartialOrd,
{
    a > b
}

#[test]
fn test_nowhere() {
    let result = compare_nowhere(3, 7);
    println!("Result: {}", result); // 输出: Result: false
}

#[test]
fn test_owhere() {
    let result = compare_where(3, 7);
    println!("Result: {}", result); // 输出: Result: false
}

#[test]
fn test_whereonly() {
    trait PrintInOption {
        fn print_in_option(self);
    }

    // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
    // 或者改用另一种间接的方法。
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
        // 否则我们会给出错误的约束。
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
