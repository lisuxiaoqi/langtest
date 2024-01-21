/*
    输出测试
    格式化输出的三种打印方式：
        {}，使用display trait
        {:?},使用debug trait,输出格式紧凑
        {:#?},使用debug trait，输出格式美化
*/
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Person<'a>{
    age:u32,
    name:&'a str
}

#[test]
fn test_fmt() {
    let p = Person{age:18, name:"zs"};
    //使用debug trait，输出：
    //Person { age: 18, name: "zs" }
    println!("{:?}",p);

    //使用debug trait，输出：
    //Person {
    //     age: 18,
    //     name: "zs",
    // }
    println!("{:#?}",p);

    //使用display trait
    //需要先手动实现对应接口
    impl fmt::Display for Person<'_>{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f,"formatted:({},{})",self.age,self.name)
        }
    }
    println!("{}",p);
}
