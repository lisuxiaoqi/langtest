/*
    循环的使用：
        loop
        while
        for
        break
        continue
        标签
*/

#[test]
fn test_loop() {
    let mut i = 1;
    //使用标签
    'inner: loop{
        if i >= 5{
            //break到指定标签
            break 'inner;
        }

        i += 1;
    }
    println!("End loop, i:{}", i);

    //loop作为表达式，用break返回值
    let mut t = 0;
    let j = loop{
        if t >= 5{
            //break语句后面加分号与不加分号效果是一样的
            break 32;
            //break 32
        }

        t += 1;
        //continue语句
        continue;
    };

    println!("End loop, j:{}", j);
}

#[test]
fn test_while(){
    let mut i = 0;
    //常规用法
    while i < 5{
        i += 1;
    }
    println!("End loop, i:{}", i);

    //while语句也可以使用标签
    i = 0;
    'inner:while i < 5{
        i += 1;

        if i == 3{
            break 'inner;
        }
        //continue语句
        continue;
    }
    println!("End loop, i:{}", i);

    //while语句不能像loop一样，由break返回结果
    //
    // i = 0;
    // let b = while i < 5{
    //     i += 1;
    //
    //     if i == 3{
    // 这种用法不支持
    //         break 30;
    //     }
    // };
}

#[test]
fn test_for(){
    //按次数循环语法，半开区间[1,5)
    for i in 1..5{
        println!("半开区间[1,5),i:{}", i);
    }

    //按次数循环语法2，闭区间[1,5]
    for i in 1..=5{
        println!("闭区间[1,5], i:{}", i);
    }

    //按次数循环语法，可以指定step
    for i in (1..5).step_by(2){
        println!("step2, i:{}", i);
    }
}

//测试for和iterator的使用
#[test]
fn test_for_iter(){
    //不通过引用方式使用集合，所有权转移
   let v = vec![1,2,3,4,5];
    for item in v{
        println!("for in: item:{}", item)
    }
    //所有权转移后，不能再使用集合
    //println!("Cannot use v after for in vec:{:?}", v);

    //通过引用方式使用集合，不会所有权转移
    let mut v = vec![1,2,3,4,5];
    for item in &v{
        println!("for in: ref usage:{}", item)
    }
    //可以继续使用集合
    println!("Can use v after for in:{:?}", v);

    //可以通过mut方式修改集合
    let mut v = vec![1,2,3,4,5];
    for item in &mut v{
        *item +=1;
        println!("for in: mut usage:{}", item)
    }
    //可以继续使用集合
    println!("Can use v after for in:{:?}", v);
}

//测试for和iterator的使用
//for item in collection        等同于 for item in collection.into_iter()	            转移所有权
// for item in &collection	    等同于 for item in collection.iter()	                    不可变借用
// for item in &mut collection	等同于 for item in collection.iter_mut()	                可变借用
#[test]
fn test_for_iter2(){
    //into_iter,不通过引用方式使用集合，所有权转移
    let v = vec![1,2,3,4,5];
    for item in v.into_iter(){
        println!("for in: item:{}", item)
    }
    //所有权转移后，不能再使用集合
    //println!("Cannot use v after for in vec:{:?}", v);

    // 通过iter()引用方式使用集合，不会所有权转移
    let mut v = vec![1,2,3,4,5];
    for item in v.iter(){
        println!("for in: ref usage:{}", item)
    }
    //可以继续使用集合
    println!("Can use v after for in:{:?}", v);

    //可以通过iter_mut方式修改集合
    let mut v = vec![1,2,3,4,5];
    for item in v.iter_mut(){
        *item +=1;
        println!("for in: mut usage:{}", item)
    }
    //可以继续使用集合
    println!("Can use v after for in:{:?}", v);
}