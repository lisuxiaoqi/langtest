/*
范型就是一种参数类型，可以加简单理解为其他语言中的interface类型
    * 定义了trait bound的就是指定interface
    * 没有定义trait bound的就是interface{}
定义Trait的时候可以使用范型
在定义Struct的时候可以使用范型

范型Struct实现范型Trait，可以达到范围很广的实现，叫做blanket implementation，像摊子一样覆盖大一片。
*/

trait AddTrait<T> {
    type Output;
    fn add(&self, other: T) -> Self::Output;
}

struct MyTrait<U> {
    value: U,
    add: u32,
}

impl<T, U> AddTrait<T> for MyTrait<U>
where
    T: Into<u32>,
    U: Copy,
{
    type Output = MyTrait<U>;
    fn add(&self, other: T) -> MyTrait<U> {
        let inner = self.add + other.into();
        MyTrait {
            value: self.value,
            add: inner,
        }
    }
}

#[test]
fn test_trait() {
    let t = MyTrait { value: "hello", add: 123 };
    let ret = t.add(100u16);
    println!("{}, {}", ret.value, ret.add);
}

