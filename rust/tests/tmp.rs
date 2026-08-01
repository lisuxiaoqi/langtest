use std::marker::PhantomData;

struct Door<T> {
    _marker: PhantomData<T>,
}

struct Open;
struct Closed;

impl Door<Open> {
    fn new() -> Door<Open> {
        Door::<Open> {
            _marker: PhantomData::default(),
        }
    }

    fn close(&self) -> Door<Closed> {
        Door::<Closed> {
            _marker: PhantomData,
        }
    }
}

#[test]
fn test() {
    let d = Door::new();
    d.close();
}
