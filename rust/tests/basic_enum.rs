#[derive(Debug)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Lowest = 0b001,
    SMI = 0b010,
    RemoteRead = 0b011,
    NMI = 0b100,
    Init = 0b101,
    Startup = 0b110,
    External = 0b111,
}

impl From<u8> for DeliveryMode {
    fn from(v: u8) -> Self {
        match v {
            0 => DeliveryMode::Fixed,
            1 => DeliveryMode::Lowest,
            2 => DeliveryMode::SMI,
            3 => DeliveryMode::RemoteRead,
            4 => DeliveryMode::NMI,
            5 => DeliveryMode::Init,
            6 => DeliveryMode::Startup,
            _ => DeliveryMode::External,
        }
    }
}

impl From<DeliveryMode> for u8 {
    fn from(v: DeliveryMode) -> Self {
        match v {
            DeliveryMode::Fixed => 0,
            DeliveryMode::Lowest => 1,
            DeliveryMode::SMI => 2,
            DeliveryMode::RemoteRead => 3,
            DeliveryMode::NMI => 4,
            DeliveryMode::Init => 5,
            DeliveryMode::Startup => 6,
            _ => 7,
        }
    }
}

impl DeliveryMode {
    pub fn get(&self) -> DeliveryMode {
        let i = 3u8;
        i.into()
    }

    pub fn set(&self, v: DeliveryMode) -> u8 {
        v.into()
    }
}

#[test]
fn enum_cast() {
    let e = DeliveryMode::Fixed;
    println!("getter:{:?}", e.get());
    println!("setter:{:?}", e.set(DeliveryMode::Init));
}
