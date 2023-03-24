use std::fmt;
use std::fmt::{Formatter, Write};

enum IpAddrKind {
    V4(u8, u8, u8, u8), // different types allowed for storing
    V6(String),
}

impl IpAddrKind {
    fn is_localhost(&self) {
        match self {
            IpAddrKind::V4(a, b, c, d) => {
                if *a == 127 && *b == 0 && (*c == 0 || *c == 1) && *d == 1 {
                    println!("IPv4 localhost address = {}.{}.{}.{}", a, b, c, d);
                } else {
                    println!("IPv4 not a localhost address = {}.{}.{}.{}", a, b, c, d);
                }
            }
            IpAddrKind::V6(address) => {
                if address == "0000:0000:0000:0000:0000:0000:0000:0001" || address == "::1" {
                    println!("IPv6 localhost address = {}", address);
                } else {
                    println!("IPv6 not localhost address = {}", address);
                }
            }
        }
    }
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IpAddrKind::V4(a, b, c, d) => {
                let arr: [u8; 4] = [*a, *b, *c, *d];
                let mut str: String = String::new();

                for (i, n) in arr.iter().enumerate() {
                    if i < 3 {
                        write!(str, "{}.", n).expect("");
                    } else {
                        write!(str, "{}", n).expect("");
                    }
                }

                write!(f, "{}", str)
            }
            IpAddrKind::V6(address) => {
                write!(f, "{}", address)
            }
        }
    }
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
    let a1: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let a2: IpAddrKind = IpAddrKind::V4(127, 0, 1, 1);
    let a3: IpAddrKind = IpAddrKind::V4(192, 168, 1, 1);
    // ipv6
    let a4: IpAddrKind = IpAddrKind::V6(String::from("madeupip"));
    let a5: IpAddrKind = IpAddrKind::V6(String::from("0000:0000:0000:0000:0000:0000:0000:0001"));
    let a6: IpAddrKind = IpAddrKind::V6(String::from("::1"));

    a1.is_localhost();
    a2.is_localhost();
    a3.is_localhost();
    a4.is_localhost();
    a5.is_localhost();
    a6.is_localhost();

    let arr: [IpAddrKind; 6] = [a1, a2, a3, a4, a5, a6];
    for index in 0..arr.len() {
        println!("i: {}, ip: {}", index, arr[index]);
    }
}
