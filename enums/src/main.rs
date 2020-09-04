#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ip_v4 = IpAddr::V4(127, 0, 0, 1);
    let ip_v6 = IpAddr::V6(String::from("::1"));
    println!("{:#?}", ip_v4);
}
