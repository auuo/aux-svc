use std::net::{IpAddr, Ipv6Addr, UdpSocket};
use chrono::Utc;
use lazy_static::lazy_static;

const VERSION: &'static str = "01";

lazy_static! {
    static ref FORMAT_IP: String = {
        let v6 = match local_ip() {
            Some(IpAddr::V4(v4)) => v4.to_ipv6_mapped(),
            Some(IpAddr::V6(v6)) => v6,
            None => Ipv6Addr::from(0),
        };
        format!("{:032x}", u128::from(v6))
    }
}

/// 生成一个用于追踪作用的 log id
/// 格式: 2 位版本号 + 13 位时间戳 + 32 位 ip + 6 位随机数 = 53 位
pub fn gen_log_id() -> String {
    let mut id = String::new();
    id.push_str(VERSION);
    id.push_str(&Utc::now().timestamp_millis().to_string());
    id.push_str(&FORMAT_IP);
    // 随机数，转 16 进制
    id
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
fn test_gen_log_id() {
    println!("{}", gen_log_id);
}