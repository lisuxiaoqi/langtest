/*
*/

//slice的下标必须是low..hight，如果low<hight,  则会出错
#[test]
fn test_slice() {
    let t = &[1, 2, 3, 4, 5];
    println!("{:?}, {:?}", &t[1..2], &t[2..]);
}

