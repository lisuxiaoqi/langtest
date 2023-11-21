fn add_two(a: i32) -> i32 {
   a+2
}

#[test]
fn test_hello() {
    assert_eq!(4, add_two(2));
    println!("Hello Test")
}