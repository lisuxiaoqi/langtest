/*
    生命周期的使用
        * 编译器需要保证引用的声明周期，不超过对象的存活时间
        * 有显示生命周期和隐式生命周期
        * 静态引用生命周期则说明该引用全程序有效
*/
struct MyStruct<'a> {
    data: &'a str,
}

#[test]
fn test_lifetime() {
    let string_data = String::from("Hello, Rust!");
    let my_struct_instance;

    {
        let reference = &string_data;
        my_struct_instance = MyStruct { data: reference };
    }

    // 在这里，string_data 已经离开作用域，但是 my_struct_instance 中仍然包含对 string_data 的引用。
    // 由于生命周期参数 'a，Rust 确保在引用的生命周期内访问引用的数据是安全的。
    println!("{}", my_struct_instance.data);
}

#[test]
fn test_static() {
    let static_ref: &'static str;
    //let static_ref : &str;
    static_ref = "hello";

    println!("{}", static_ref)
}

/*
    impl可以按照自己的需要声明需要的生命周期，不依赖于struct定义中是否包含生命周期
*/
#[test]
fn test_impl_lifetime() {
    struct Container<T> {
        data: T,
    };

    impl<'a> Container<&'a str> {
        fn get(&self) -> &str {
            self.data
        }
    };

    let s = String::from("hello");
    let str = s.as_str();
    let c = Container { data: str };
    print!("{}", c.get());
}

/*
 *variance:
 * covariance: 长生命周期可以当短生命周期使用，仅限于只读
 * invariance：长生命周期不能当短生命周期使用，比如可写mut
 * */

#[test]
fn test_variance() {
    //long lifetime
    let long_str: &'static str = "hello";
    {
        //short lifetime
        let short_string = String::from("world");
        let short_str = short_string.as_str();
        let mut short_ptr = &short_str;
        let long_ptr = &long_str;

        //assign to long lifetime is legal when readonly
        short_ptr = long_ptr;
        println!("short life str:{}", short_ptr);
    }

    //long lifetime
    let mut long_str2: &'static str = "hello";
    {
        //short lifetime
        let short_string = String::from("world");
        let mut short_str = short_string.as_str();
        let mut short_ptr = &mut short_str;
        let long_ptr = &mut long_str2;

        //assign to long lifetime is legal when readonly
        short_ptr = long_ptr;
        println!("short life str:{}", short_ptr);
    }
    println!("long life str:{}", long_str);
}

/*
 *reborrow: 引用在函数传递的时候，往往会被通过reborrow转化，
 * 把原来的mut引用冻结，然后创建新的mut引用，
 * 过程中不会涉及到variance原则
 * */
#[test]
fn test_reborrow() {
    let mut raw = 32;
    {
        let ra = &raw;
        let ptr_ra = ra as *const i32 as *const u8;
        println!("address of ra:{:p}", ptr_ra);

        let rb = &raw;
        let ptr_rb = rb as *const i32 as *const u8;
        println!("address of rb:{:p}", ptr_rb);
    }

    //可写引用和可读引用都指向相同的内存
    let wa = &mut raw;
    let ptr_wa = wa as *const i32 as *const u8;
    println!("address of wa:{:p}", ptr_wa);

    //here reborrow,reborrow对wa创建新的引用，但是指向的不是wa，而是raw的内存地址
    //注意，此时wa被冻结，因此也不违背raw不能多个mut引用的原则
    let reb_of_wa = &mut *wa;
    let ptr_reb_of_wa = reb_of_wa as *const i32 as *const u8;
    println!("address of reborrow of wa:{:p}", ptr_reb_of_wa);

    //wa was frozen in the lifetime of reb_of wa
    //println!("wa was frozen:{}", *wa);

    println!("reb_of_wa points to value:{}", *reb_of_wa);
}
