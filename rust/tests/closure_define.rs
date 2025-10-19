/*
    闭包的使用测试

        闭包的定义
        闭包capture变量
        Fn, FnOnce, FnMut

        Fn:
            * 不能修改外部变量
        FnMut:
            * 可以修改闭包捕获的外部变量
        FnOnce:
            * 只能使用一次
*/

//测试闭包函数的显示定义和隐式定义
//* 显式：指定输入输出类型
//* 隐式：不指定类型
#[test]
fn test_define() {
    //显式定义闭包函数
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("add result:{}", add(1, 3));

    //隐式定义闭包函数
    let add2 = |x, y| x + y;
    println!("add2 result:{:?}", add2(1, 3));

    //部分定义类型
    let add3 = |x: i32, y| -> i32 { x + y };
    println!("add3 result:{:?}", add3(1, 3));
}


#[test]
fn test_fnmut() {
    let mut env_var = 0;

    //定义一个修改外部变量的闭包
    let mut_add = |x| {
        env_var += 1;
        x + env_var
    };

    //  定义传入闭包的函数
    // f要定义为mut，它修改了外部环境变量
    fn add<F>(mut f: F, n: i32) -> i32
    where
        F: FnMut(i32) -> i32,
    {
        f(n) + f(n)
    }

    let ret = add(mut_add, 10);
    println!("{},{}", ret, env_var);
}

#[test]
fn test_fnonce() {
    let env_var = String::from("Hello World");
    let onceFn = || {
        let len = env_var.len();
        println!("{}", len);
        //当闭包中有明显的move动作时，会发生move
        //即使闭包没有声明move
        let _s = env_var;
    };

    onceFn();
    //使用env_var会失败
    //println!("env:{}", env_var);
}

#[test]
fn test_fnonce2() {
    let env_var = String::from("Hello World");
    let onceFn = move || {
        //当出现move关键字，即使这样的只读借用，也会导致所有权转移
        let len = env_var.len();
        println!("{}", len);
    };

    onceFn();
    //使用env_var会失败，即使闭包中只读，但是使用了move关键字
    //导致所有权转移
    //println!("env:{}", env_var);
}