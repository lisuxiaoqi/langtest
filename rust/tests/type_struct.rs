/*
    struct的基础使用
        struct分类：
            * C struct，常规结构体，包括属性和方法
            * 单元结构体，unit struct。用于不需要属性，仅需要方法的场景
            * 元组结构体，tuple struct。用于不需要给属性命名的场景

        self的区别：
            * Self,一般用于new，代表struct本身
            * &self,可变借用
            * &mut self, 不可变借用
            * rust会使用自动引用和解引用的功能。当使用object.something() 调用方法时，
                Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配
*/

//C Struct
pub struct Rectangle {
    //private field
    width: u32,
    height: u32,
}

impl Rectangle {
    //new是关联函数，没有self。new不是rust关键字，约定为创建Object
    //Self关键字。Self指代的就是被实现方法的结构体Rectangle。
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    //getter。名字一般和field相同
    //&self。不可变借用
    pub fn width(&self) -> u32 {
        return self.width;
    }
}

// 一个struct可以有多个impl
impl Rectangle {
    pub fn height(&self) -> u32 {
        return self.height;
    }

    //&mut self。可变借用
    pub fn add_height(&mut self) {
        self.height += 1;
    }
}

#[test]
fn test_cstruct() {
    //通过::使用关联函数
    let mut r = Rectangle::new(10, 10);

    //不可变借用
    println!("getter,width:{}", r.width());

    //采用可变借用
    r.add_height();
    println!("getter,height:{}", r.height());
}

//单元结构体用于仅需要方法，不需要属性的场景
//比如仅用于实现一个trait
#[test]
fn test_unit_struct() {
    //声明单元结构体。
    struct UnitStruct;
    impl UnitStruct {
        fn foo(&self) {
            println!("Unit Struct, Only method is needed")
        }
    }

    //声明单元结构体变量
    let s = UnitStruct;
    s.foo();
}

//元组结构体用于不需要属性名称的场景
//比如大家都知道颜色用RGB定义，那么声明color时就不需要指定属性名称
//元组结构体性质和tuple一样，就是给tuple起了个名字,通过索引访问成员
#[test]
fn test_tuple_struct() {
    //声明结构体。
    struct Color(i32, i32, i32);
    impl Color {
        fn foo(&self) {
            println!(
                "Tupple Struct, field name is unnecessary,{},{},{}",
                self.0, self.1, self.2
            )
        }
    }
    //声明元组结构体变量
    let s = Color(128, 128, 128);
    s.foo();

    //解析结构体到具体变量
    let Color(r, g, b) = s;
    println!("{},{},{}", r, g, b)
}
