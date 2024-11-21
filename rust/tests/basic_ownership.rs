/*
    所有权测试
    所有权中的copy, clone和move:
        * copy。赋值时，栈上的数据采用copy方式，即原有数据还在，创建了一份新的数据。几乎栈上的数据都属于copy，如下：
            所有整数类型，比如 u32
            布尔类型，bool，它的值是 true 和 false
            所有浮点数类型，比如 f64
            字符类型，char
            元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
            不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的,而是move

        * move。赋值时，堆上的数据采用move方式，原有数据所有权转移到新的变量中，原有变量丧失所有权，比如String,可变引用
        * clone。 可对堆上和栈上数据使用，clone是拷贝一份新的数据，原数据还在。
                  clone需要手动定义clone函数， 手动调用，目的是执行深拷贝。

        * copy, move在赋值，或者函数传参数，获取返回值时，自动执行
*/

#[test]
fn test_ownership() {
    //i32: copy
    let i1 = 3;
    let i2 = i1;
    println!("copy i32, i1:{}, i2:{}", i1, i2);

    //reference:copy
    let r1: &str = "hello";
    let r2 = r1;
    println!("copy reference, r1:{:?}, r2:{:?}", r1, r2);

    let mut m1: i32 = 5;
    {
        let rm1: &mut i32 = &mut m1;
        println!("mut reference,  rm1:{:?}", rm1);
        let rm2: &mut i32 = rm1;
        //rm1的生命周期结束，对于可变引用，使用的是move
        //println!("copy reference, rm1:{:?}, rm2:{:?}", rm1, rm2);
        println!("move mut reference, rm2:{:?}", rm2);
    }
    println!("origin mut,  m1:{:?}", m1);

    //tuple: copy
    let t1: (i32, &str) = (1, "hello");
    let t2 = t1;
    println!("copy tuple with reference, t1:{:?}, t2:{:?}", t1, t2);

    //String:move
    let s1: String = String::from("hello");
    let s2 = s1;
    //println!("move, a:{:?}", s1);
    //^ s1所有权已转移，因此不能再使用s1
    println!("move String, s2:{:?}", s2);

    //tuple: move。tuple中含有String，属于堆上数据，因此是move
    let t1: (i32, String) = (1, "hello".to_string());
    let t2 = t1;
    println!("move tuple with heap, t2:{:?}", t2);

    //clone需要显式调用。clone之后原数据存在
    //cloned String，堆上数据
    let c1: String = String::from("hello");
    let c2 = c1.clone();
    println!("clone string, c1:{}, c2:{}", c1, c2);

    //cloned i32，栈上数据
    let c1: i32 = 45;
    let c2 = c1.clone();
    println!("clone i32, c1:{}, c2:{}", c1, c2);

    //copy, move in function call
    let pStack: i32 = 40;
    let pHeap: String = String::from("hell0");
    let (r1, r2) = foo(pStack, pHeap);
    //println!("Head ownership moved:{}", pHeap);
    //^ 执行错误，pHead的所有权已经被move了

    //返回时，r1采用copy方式，r2采用move方式
    println!("function return value, copy:{}, move:{}", r1, r2);
}

//调用时，p1采用copy方式，p2采用move方式
fn foo(p1: i32, p2: String) -> (i32, String) {
    println!("function received value, copy:{}, move:{}", p1, p2);
    (p1, p2)
}
