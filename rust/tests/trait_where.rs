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
fn test_where() {
    let result = compare_where(3, 7);
    println!("Result: {}", result); // 输出: Result: false
}

#[test]
fn test_whereonly() {
    trait PrintInOption {
        fn print_in_option(self);
    }

    // 我们想要的是Option<T>: Debug，因此这里需要一个 `where` 从句
    // 不能表达成 `T: Debug`
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // 我们想要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
        // 而不是写成T: Debug。
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
