/*
   Seal是一种设计模式，用于限制trait的可访问范围
   例如trait private仅想在mod内部使用，而trait public需要对外暴露，
   可trait public依赖于trait private，即 public：private
   当对外暴露public时，private也会连带暴露，因此使用seal模式，即限制private范围设置为mod内部使用，
   在对外public：private时，由于private时对外不可见，外部只需要实现public的方法
*/

use crate::outer::public;

// mod outer提供了trait public对外使用
mod outer {
    // trait B对外暴露，
    // 但由于依赖于trait A,如果不做处理，外部实现在实现B时，也需要实现A的函数
    // 为了不暴露trait A，把trait A放在一个private的mod inner中
    pub trait public: inner::private {
        fn foo(&self);
    }

    //mod inner是private的，不对外暴露
    mod inner {
        pub trait private {
            fn seal(&self);
        }
    }

    //这步很关键，对实现了trait public的，添加一个默认实现
    impl<T: public> inner::private for T {
        //默认实现，可以什么都不做
        //这样外部在实现trait public的的时候，就不用
        fn seal(&self) {}
    }
}

struct S;

impl outer::public for S {
    fn foo(&self) {
        println!("only public trait is implemented")
    }
}

#[test]
fn test_seal() {
    let s = S;
    s.foo();
}
