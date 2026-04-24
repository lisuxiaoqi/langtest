/*
 * use Miri to help checking unsafe block
 *      * rustup +nightly component add miri rust-src
 *      * cargo +nightly miri setup --verbose
 *      * Cargo +nightly miri test --test basic_unsafe -- --show-output
 *      * Cargo +nightly miri test --test basic_unsafe test_linked_list -- --show-output
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

/*
 *  证明unsafe可以修改绑定了只读的引用数据，但需要先绑定mut指针，再绑定只读，
 *  反之先绑定只读，编译器就会聪明的阻止后续的绑定mut指针
 * */
#[test]
fn test_alias_violation() {
    let mut raw = 10;
    //get pointer by macro
    let ptr = std::ptr::addr_of_mut!(raw);
    //or get pointer by cast
    let ptr2 = &mut raw as *mut i32;

    //set an read only reference, but it can be changed by unsafe code
    let r = &raw;

    unsafe {
        *ptr = 20;
        *ptr2 += 5;
    }
    println!("value through ref:{}", r);
}

#[test]
fn test_linked_list() {
    struct Node {
        value: i32,
        next: *mut Node,
    }

    let mut n1 = Node {
        value: 30,
        next: std::ptr::null_mut(),
    };
    let mut n2 = Node {
        value: 60,
        next: std::ptr::null_mut(),
    };
    //使用裸指针是安全的，可以不用放入unsafe
    n1.next = &mut n2 as *mut Node;

    let mut cur: *mut Node = &mut n1;
    while !cur.is_null() {
        unsafe {
            //但解引用裸指针是unsafe的，必须放入unsafe块，因为编译器不知道内存是否有效
            println!("Cur value:{}", (*cur).value);
            cur = (*cur).next;
        }
    }
}

/*
 *  unsafe trait是一种软约定，开发者申明unsafe trait, 表明该trait有必须遵守的规范
 *  实现者标注unsafe impl表明自己遵循了规范，仅此而已，编译器无其他额外检查
 * */
unsafe trait Trans {}
struct Point {
    _x: i32,
    _y: i32,
}
unsafe impl Trans for Point {}

fn handle_trans<T: Trans>(data: &T) {
    let ptr: *const u8 = data as *const T as *const u8;
    let size = std::mem::size_of::<T>();
    unsafe {
        let bytes = std::slice::from_raw_parts(ptr, size);
        println!("data bytes:{:?}", bytes);
    }
}
#[test]
fn test_unsafe_trait() {
    let p = Point { _x: 10, _y: 20 };
    handle_trans(&p);
}

union RawInt {
    i: u32,
    b: [u8; 4],
}
#[test]
fn test_union() {
    let ri = RawInt { i: 1 << 16 };
    unsafe {
        println!("union in u32:{}", ri.i);
        println!("union in bytes:{:?}", ri.b);
        let p: *const u8 = &ri as *const RawInt as *const u8;
        println!("union in first byte, addr:{:p}, val:{:?}", p, *p);
        let p1 = p.add(1);
        println!("union in second byte, addr:{:p}, val:{:?}", p1, *p1);
        let p2 = p.add(2);
        println!("union in third byte, addr:{:p}, val:{:?}", p2, *p2);
        let p3 = p.add(3);
        println!("union in forth byte, addr:{:p}, val:{:?}", p3, *p3);
    }
}

#[test]
fn test_endian() {
    let i = 1u32;
    let p: *const u8 = &i as *const u32 as *const u8;
    unsafe {
        match *p {
            1 => println!("small endian"),
            0 => println!("big endian"),
            _ => (),
        }
    }
}
