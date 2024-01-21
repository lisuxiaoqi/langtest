/*
    类型转换测试

    可使用as关键字类型转换，rust没有隐式类型转换
*/
// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

#[test]
fn test_as() {
    let decimal = 65.6321_f32;
    //把decimal转化为int，抛弃小数点后面数据
    let integer = decimal as u8;
    //把integer转化为char
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    //有符号负数转化为正数，就是取补码
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // -2 + 256 = 254
    println!("  -2 as a u8 is : {}", (-2i8) as u8);

    //超过范围的数转化，只保留低位和符号
    //257>255
    //结果和取模一样。257 % 256
    println!("  257 as a u8 is : {}", 257 as u8);
}
