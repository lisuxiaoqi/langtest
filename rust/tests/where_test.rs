/*
    测试where的用法:
        where用于描述trait，目的是使语法更为直观
        where可以被T：Type方式替代
*/

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
