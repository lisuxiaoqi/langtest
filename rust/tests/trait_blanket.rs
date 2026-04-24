/*
 * Blanket为一批实现了Mannual的类型，自动实现AutoT类型
*/
trait AutoT {
    fn auto();
}

trait Mannual {
    fn man();
}

impl<T: Mannual> AutoT for T {
    fn auto() {
        println!("Auto impled for Mannual");
    }
}

struct ST;
impl Mannual for ST {
    fn man() {
        println!("Mannual impled for S");
    }
}

#[test]
fn blanket() {
    <ST as Mannual>::man();
    <ST as AutoT>::auto();
}
