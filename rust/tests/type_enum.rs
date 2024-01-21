/*
    enum的使用
        enum可以作为c风格的enum使用
            * c风格的enum，就是给int,uint整数起别名
            * c风格的enum，值可以隐式递增，也可以显示指定

        enum在rust中，更多是作为struct模式使用
            * enum不同于trait的抽象方式。trait的抽象，要求不同struct有相同的接口，
              而enum的抽象，不要求struct有共同点，而是不同的struct需要对外呈现为统一的类型。
            * enum成员可以包含unit struct, tuple struct和c struct
            * enum往往和match配合使用,系统中最常用的enum是Option和Result
            * enum和struct一样可以实现方法
*/

// C风格的enum，enum中成员为uint，类似于define, 为不同的uint起别名
#[test]
fn test_c_enum() {
    // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
    enum Number {
        Zero,
        One,
        Two,
    }

    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

// struct风格的enum
#[test]
fn test_struct_enum() {
    //enum中的成员为struct，每个struct没有相同点
    //仅仅需要对外统一展示为WebEvent类型
    enum WebEvent {
        // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
        PageLoad,
        PageUnload,
        // 或者一个元组结构体，
        KeyPress(char),
        Paste(String),
        // 或者一个普通的结构体。
        Click { x: i64, y: i64 },
    }

    // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
    // 内部处理enum时，采用match方式
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // 从 `enum` 里解构出 `c`。
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // 把 `Click` 解构给 `x` and `y`。
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    //创建不通的enum成员类型
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    //交给inspect统一按照WebEvent方式处理
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
