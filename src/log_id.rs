use std::net::{IpAddr, Ipv6Addr, UdpSocket};
use chrono::Utc;

const VERSION: &'static str = "01";

/// 生成一个用于追踪作用的 log id
pub fn gen_log_id() -> String {
    let mut id = String::new();
    id.push_str(VERSION);
    id.push_str(&Utc::now().timestamp_millis().to_string());
    // local ip 编码为 32 位长度的十六进制
    // 随机数，转 16 进制
    id
}

fn a() -> String {
    let v6 = match local_ip() {
        Some(IpAddr::V4(v4)) => v4.to_ipv6_mapped(),
        Some(IpAddr::V6(v6)) => v6,
        None => Ipv6Addr::from(0),
    };
    format!("{:02x}", u128::from(v6))
}

/// 获得本地 ip，并不会真的发起请求，使用 udp 也不会有握手请求
fn local_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().map(|addr| addr.ip()).ok()
}

#[test]
fn test_get_local_ip() {
    println!("{:?}", local_ip());
}

#[test]
fn test_get_ip_hex() {
    println!("{}", a())
}