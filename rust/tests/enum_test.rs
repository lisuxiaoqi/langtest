/*
    enum的使用
        * enum的成员其实是一个个struct，有unit struct, tuple struct和c struct
        * enum可以作为c风格的enum使用
        * enum更多扮演一个父类的角色，c++中的继承关系在go,rust中不复存在，取而代之的是接口，
            但接口要求子类实现统一的method，如果要求调用不同子类的不同method则无能为力。
            比如有一个animal,可能是tiger，也可能是dog。如果是tiger，则需要调用其hunt函数，如果是dog，则
            需要调用bark函数，此时无法用接口实现，此时enum是最佳选择。
        * enum往往和match配合使用,系统中最常用的enum是Option和Result
        * enum和struct一样可以实现方法
*/

#[test]
fn test_c_enum(){
    #[derive(Debug, Default)]
    pub enum BlockNumber {
        /// Latest block
        #[default]
        Latest,
        /// Finalized block accepted as canonical
        Finalized,
        /// Safe head block
        Safe,
        /// Earliest block (genesis)
        Earliest,
        /// Pending block (not yet part of the blockchain)
        Pending,
        /// Block by number from canon chain
        Number(u64),
    }
    let b = BlockNumber::Finalized;
    println!("Block:{:?}", b)
}