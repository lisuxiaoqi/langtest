use std::marker::PhantomData;

struct Disconnected;
struct Connected;
struct Authenticated;

struct Connection<State> {
    socket: i32,

    _marker: PhantomData<State>,
}

impl Connection<Disconnected> {
    fn new() -> Self {
        Self {
            socket: 0,
            _marker: PhantomData,
        }
    }

    fn connect(self) -> Connection<Connected> {
        println!("connecting...");

        Connection {
            socket: 1,
            _marker: PhantomData,
        }
    }
}

impl Connection<Connected> {
    fn authenticate(self) -> Connection<Authenticated> {
        println!("auth success");

        Connection {
            socket: self.socket,
            _marker: PhantomData,
        }
    }
}

impl Connection<Authenticated> {
    fn send(&self, msg: &str) {
        println!("sending: {}", msg);
    }
}

#[test]
fn test_typestate() {
    let conn = Connection::<Disconnected>::new();
    let conn = conn.connect();
    let conn = conn.authenticate();
    conn.send("hello world");
}
