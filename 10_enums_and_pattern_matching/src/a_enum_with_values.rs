enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn example1() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    let localhost = IpAddrKind {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
}
