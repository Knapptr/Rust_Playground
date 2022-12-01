#[allow(dead_code)]

fn main() {
    let v4 = IpAddrKind::v4("127.0.0.1".to_string());
    let v6 = IpAddrKind::v6("::1".to_string());
    print_ip(&v4);
    print_ip(&v6);
    print_ip(&v4);
}

enum IpAddrKind {
    v4(String),
    v6(String),
}

fn print_ip(ip: &IpAddrKind) {
    match ip {
        IpAddrKind::v4(s) => println!("V4 ip: {}", s),
        IpAddrKind::v6(s) => println!("V6 ip: {}", s),
    }
}
