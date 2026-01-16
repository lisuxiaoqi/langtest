mod pub_trait {
    mod private {
        pub(super) trait Seal {}
    }

    #[allow(private_bounds)]
    pub trait Interface: private::Seal {
        fn foo(&self);
    }

    pub struct SealStruct;
    impl private::Seal for SealStruct {}

    impl Interface for SealStruct {
        fn foo(&self) {
            println!("Interface trait get called");
        }
    }
}

#[test]
fn test_seal() {
    use pub_trait::*;
    let s = SealStruct {};
    <SealStruct as Interface>::foo(&s);
}
