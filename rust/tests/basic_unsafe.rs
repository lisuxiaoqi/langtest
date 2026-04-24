/*
 * use Miri to help checking unsafe block
 *      * rustup +nightly component add miri rust-src
 *      * cargo +nightly miri setup --verbose
 *      * Cargo +nightly miri test --test basic_unsafe -- --show-output
 *
 * */

//测试裸指针在rust中的用法
#[test]
fn test_basic_pointer() {
    //const pointer
    let cp: *const i32 = &10;
    println!("const pointer address is: {:p}", cp);

    // mut pointer
    let mut i: i32 = 10;
    let mp: *mut i32 = &mut i;
    println!("mut pointer address is: {:p}", mp);
    unsafe {
        *mp = 20;
        println!("value changed by pointer: {}", *mp);
    }
}

unsafe fn swap_raw(pa: *mut i32, pb: *mut i32) {
    let c = *pa;
    *pa = *pb;
    *pb = c;
}

#[test]
fn test_swap() {
    let mut a = 10i32;
    let mut b = 20i32;
    let pa: *mut i32 = &mut a;
    let pb: *mut i32 = &mut b;
    unsafe {
        println!("before swap raw,a:{}, b:{}", *pa, *pb);
        swap_raw(pa, pb);
        println!("before swap raw,a:{}, b:{}", *pa, *pb);
    }
}

#[test]
fn test_alloc() {
    let l = std::alloc::Layout::new::<i32>();
    unsafe {
        let rawp = std::alloc::alloc_zeroed(l);
        if rawp.is_null() {
            panic!("failed to alloc mem");
        }
        let u32_ptr = rawp as *mut i32;
        *u32_ptr = 32;
        println!("u32 ptr value is:{}", *u32_ptr);
        std::alloc::dealloc(rawp, l);
    }
}
