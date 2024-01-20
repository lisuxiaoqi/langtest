/*
    Rc智能指针的使用：
        1. Rc是只读的
        2. Rc的可写需要配合Mutex或者Refcell
*/

use std::rc::Rc;

//Rc可以多个所有者，是单线程只读的
#[test]
fn test_Rc() {
   let shared = Rc::new(10);
    {
        //使用Rc::clone
        let s1 = Rc::clone(&shared);
        println!("Rc clone1:{}", s1);
    }
    {
        //使用obj.clone，二者是一样的
        let s2 = shared.clone();
        println!("Rc clone2:{}", s2);
    }
    println!("Rc object:{}", shared);
}

//Rc可写需要配合Refcell
#[test]
fn test_write_Rc() {
    let shared = Rc::new(10);
    {
        //使用Rc::clone
        let s1 = Rc::clone(&shared);
        println!("Rc clone1:{}", s1);
    }
    {
        //使用obj.clone，二者是一样的
        let s2 = shared.clone();
        println!("Rc clone2:{}", s2);
    }
    println!("Rc object:{}", shared);
}
