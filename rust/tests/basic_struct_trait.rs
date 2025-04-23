/*
    定义一个struct
        * 使用new创建
        * 使用Default trait可以设置默认值
        * 默认成员变量和方法都是private，使用pub可以设置为public
*/

//使用Default trait,必须声明#[derive(Default)]
mod inner_mode {
    #[derive(Default)]
    pub struct Person {
        name: String,     //private
        age: u32,         //private
        pub gender: bool, //public
    }

    impl Person {
        pub fn new(name: String, age: u32, gender: bool) -> Self {
            Self { name, age, gender }
        }

        pub fn new_with_default(name: String) -> Self {
            //除了name剩下其他的变量，采用default赋值
            Self {
                name,
                ..Default::default()
            }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn age(&self) -> u32 {
            self.age
        }
    }
}

#[test]
fn test_struct_new() {
    let p1 = inner_mode::Person::new(String::from("John"), 27, true);
    println!(
        "p1 name is:{}, age is:{}, gender is:{}",
        p1.name(),
        p1.age(),
        p1.gender
    );

    let p2 = inner_mode::Person::new_with_default(String::from("Jack"));
    println!(
        "p2 name is:{}, age is:{}, gender is:{}",
        p2.name(),
        p2.age(),
        p2.gender
    );
}

/*
    测试..语法的使用，..叫做"Struct Update Syntax"（结构体更新语法)，从另一个结构体复制未显示赋值的字段
*/
struct Student {
    name: String,
    age: u32,
}
#[test]
fn test_struct_update_syntax() {
    let p1 = Student {
        name: String::from("John"),
        age: 27,
    };
    let p2 = Student {
        name: String::from("jack"),
        ..p1 //利用..语法从p1拷贝剩下数据
    };

    println!("p1 name is:{}, age is:{}", p1.name, p1.age);
    println!("p2 name is:{}, age is:{}", p2.name, p2.age);
}

/*
    测试trait的使用，trait类似于interface，可以被具体的struct继承
*/
