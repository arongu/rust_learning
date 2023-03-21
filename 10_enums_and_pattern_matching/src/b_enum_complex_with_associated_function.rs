enum IpAddrKind {
    V4(u8, u8, u8, u8), // different types allowed for storing
    V6(String),
}

enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // string
    ChangeColor(i32, i32, i32), // 3 integers
}

impl Message {
    fn some_function() {
        println!("Let's get Rusty!");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {}

pub fn example1() {
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
}
