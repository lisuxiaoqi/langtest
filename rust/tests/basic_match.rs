/*
   match必须覆盖所有分支,如果有多个分支匹配，第一个分支会被选择
   match的本质是destructure，即解构
   如果解构成功，则匹配，并且赋值给对应的变量

*/

#[test]
fn test_basic() {
    let number = 13;
    //熟知匹配
    match number {
        //匹配单个值
        1 => {
            println!("One!")
        }
        //匹配多个值
        1 | 2 | 3 | 13 => println!("Multiple"),
        // 匹配一个闭区间范围，虽然匹配上，但前面已经匹配过
        // 因此不会被匹配
        12..=19 => println!("Zone"),
        //匹配剩余情况
        _ => {
            println!("Ain't special")
        }
    }

    let boolean = true;
    //match可以作为表达式
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    println!("{} -> {}", boolean, binary);
}

#[test]
fn test_ref() {
    let number = 4;
    //普通解构
    match number {
        val => {
            println!("normal match:{}", val)
        }
    }

    //解构成ref
    match number {
        //ref val的意思，是把number解构成引用，并赋值给val，因此val=&number
        ref val => {
            println!("ref match:{}", *val)
        }
    }

    //效果等同于通过ref匹配
    match &number {
        val => {
            println!("ref match:{}", *val)
        }
    }

    //效果等同于通过ref匹配
    let ref_num = &number;
    match ref_num {
        val => {
            println!("ref match:{}", *val)
        }
    }

    //可以配合mut关键字使用
    let mut number = 4;
    match number {
        //把number解构成可变引用，赋值给val，因此val的类型是&mut
        ref mut val => {
            *val += 1;
        }
    }
    println!("number after ref match:{}", number)
}
