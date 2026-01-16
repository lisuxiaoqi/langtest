trait HighPro {
    fn report(self);
}
trait MidPro {
    fn report(self);
}
trait LowPro {
    fn report(self);
}

struct StructAll;
impl HighPro for &&&StructAll {
    fn report(self) {
        println!("HighPro is called");
    }
}
impl MidPro for &&StructAll {
    fn report(self) {
        println!("MidPro is called");
    }
}
impl LowPro for &StructAll {
    fn report(self) {
        println!("LowPro is called");
    }
}

struct StructPartialM;
impl MidPro for &&StructPartialM {
    fn report(self) {
        println!("MidPro is called");
    }
}

impl LowPro for &StructPartialM {
    fn report(self) {
        println!("LowPro is called");
    }
}

struct StructPartialL;
impl LowPro for &StructPartialL {
    fn report(self) {
        println!("LowPro is called");
    }
}

#[test]
fn multi_deref() {
    let s = StructAll {};
    println!("StructAll:");
    (&&&s).report();
    s.report();

    let s = StructPartialM {};
    println!("StructPartialM:");
    (&&&s).report();
    s.report();

    let s = StructPartialL {};
    println!("StructPartialL:");
    (&&&s).report();
    s.report();
}
