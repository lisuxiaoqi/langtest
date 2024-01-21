/*
    From, Into是常见的转化trait。二者都是把A转化为B
    假如要把A转化为B：
        1. 在B中实现From trait
        2. 调用B::From(A),把A转化为B
        3. 也可以调用A.into()，也是把A转化为B

    和From和Into一样，TryFrom和TryInto 是类型转换的trait。
    区别在于TryFrom 和 TryInto trait的返回值是 Result 型


    ToString trait用于把A转化为String，但一般不这么做，而是实现fmt::Display trait，它会自动提供 ToString
    FromStr trait用于把String转化为A，只要对目标类型实现了FromStr trait，就可以用 parse 把字符串转换成目标类型
*/
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
struct Number {
    value: i32,
}

//实现From trait，会自动获得into函数
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

//测试from, into
#[test]
fn test_from_info() {
    //From trait
    let num = Number::from(18);
    println!("My number is {:?}", num);

    //实现from tait就自动获得into函数
    //需要声明变量时显式指定类型 n2:Number
    let num2: Number = 17.into();
    println!("My number by into is {:?}", num2);
}

//测试TryFrom,TryInto
//功能和From, Into一样是把A转化为B，区别在于返回值为Result

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[test]
fn test_try_from_into() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

//ToString
struct Circle {
    radius: i32,
}

//实现Display trait就会自动实现ToString
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius by display {}", self.radius)
    }
}

//也可以手动实现ToString
struct Circle2 {
    radius: i32,
}
impl ToString for Circle2 {
    fn to_string(&self) -> String {
        format!("Circle of radius by ToString {:?}", self.radius)
    }
}

#[test]
fn test_tostring() {
    let circle = Circle { radius: 10 };
    println!("{}", circle.to_string());

    let circle2 = Circle2 { radius: 10 };
    println!("{}", circle2.to_string());
}
