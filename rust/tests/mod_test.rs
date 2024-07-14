/*
    mod使用测试

    mod封装一个域，可控制pub和private访问权限
    mod可以通过mod关键字显示声明
    crate名称，文件夹和文件名是隐式的mod路径
*/
//平级private mod
mod same_level {
    //private函数不能被调用
    fn private() {
        println!("private foo cannot be called")
    }

    //public函数能被调用
    pub fn public() {
        println!("public foo can be called")
    }

    //pub inner mod可以被调用
    pub mod public_inner_mod {
        pub fn public_inner() {
            println!("public_inner foo can be called in public mod")
        }

        pub fn private_inner() {
            println!("private_inner foo can be called in public mod")
        }
    }

    //private inner mod不可以被调用
    mod private_inner_mod {
        pub fn public_inner() {
            println!("public_inner foo cannnot be called in private mod")
        }
    }
}

//#[test]表示cargo test会执行该测试函数
#[test]
fn test_mod_same_level() {
    //平级mod可以直接访问，即使是private mod
    use same_level;
    same_level::public();

    //只有public的子mod可以被访问
    same_level::public_inner_mod::public_inner();
    //same_level::private_inner_mod::public_inner();
}
