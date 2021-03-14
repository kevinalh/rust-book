#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("The IP address struct is {:#?}", home);
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
