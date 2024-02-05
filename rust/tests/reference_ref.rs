/*
    ref关键字的使用：
        ref关键字在左边，以下二者是等同的：
            let ref r = 3;
            let r = &3;
*/
#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_ref() {
    //ref基本用法，ref关键字等同于&
    let ref r1 = 3;
    let r2 = &3;
    println!("ref_c1 equals ref_c2: {}", *r1 == *r2);

    //ref可以用在模式匹配中
    let point = Point { x: 10, y: 20 };
    let copy_of_x = {
        //模式匹配语法，见basic_let.rs
        //等同于let ref ref_to_x = point.x;
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };
    println!("copy_of_x:{}", copy_of_x);

    //ref和mut关键字一起使用
    let mut mutable_point = point;
    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        //mut_ref_to_y是mutable_point.y的引用,等同于:
        //let mut_ref_to_y = &mut mutable_point.y
        //修改会导致mutable_point.y变动
        *mut_ref_to_y = *mut_ref_to_y + 1;
    }
    println!("mutable_point:{:#?}", mutable_point);

    //ref和mut关键字一起使用例子2
    let mut mut_tuple = (Box::new(3u32), 5u32);
    {
        let (_, ref mut last) = mut_tuple;
        *last += 1;
    }
    println!("mut_tuple:{:#?}", mut_tuple);
}
