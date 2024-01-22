/*
   adding是一种设计模式
   A是一个对外暴露的trait，已经有了一些具体的实现如I1, I2
   此时需要对I1,I2同时添加一个新的trait B默认实现，就采用adding方法:
    impl <T:A> B for T{}
*/

//A是一个trait
trait A {
    fn foo(&self);
}

//I1,I2是A的实现
struct I1;
impl A for I1 {
    fn foo(&self) {
        println!("I1 impl for A");
    }
}
struct I2;
impl A for I2 {
    fn foo(&self) {
        println!("I2 impl for A");
    }
}

//B是一个新增加的trait，想对I1,I2提供默认实现
trait B {
    fn poo(&self);
}

//对于所有实现了A的类型T，提供trait B的默认实现
impl<T: A> B for T {
    fn poo(&self) {
        println!("all A trait impls have trait B implementation now");
    }
}

#[test]
fn test_seal() {
    let i1 = I1;
    let i2 = I2;

    i1.foo();
    i1.poo();

    i2.foo();
    i2.poo();
}
