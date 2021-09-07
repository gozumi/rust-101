enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKindConcise {
    V4(String),
    V6(String),
}

enum IpAddrKindDetailed {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_concise = IpAddrKindConcise::V4(String::from("127.0.0.1"));
    let loopback_concise = IpAddrKindConcise::V6(String::from("::1"));

    let home_detailed = IpAddrKindDetailed::V4(127, 0, 0, 1);
    let loopback_detailed = IpAddrKindDetailed::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
