//1. 每个符号，包括关键字和运算符，都被视为tt
//2. 其中被(), {}和[]包围的整体， 视为一个tt,不再进入
//3. (), {}和[]视为tt的一部分, 而不仅是里面的内容
macro_rules! tt_stringify {
    ({$($val:tt)*}) => {$(
        println!("Received tt:{}", stringify!($val));
    )*};
}
#[test]
fn test_tt() {
    tt_stringify!({
        let a = 10;
        let b = (10 + 3);
        let c = { 10 };
    });
}
