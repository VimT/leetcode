//! 验证IP地址

#[inline]
fn is_ipv4_section(s: &str) -> bool {
    if s.len() > 3 || (s.len() > 1 && s.as_bytes()[0] == b'0') { return false; }
    u8::from_str_radix(s, 10).is_ok()
}

#[inline]
fn is_ipv6_section(s: &str) -> bool {
    if s.len() > 4 { return false; }
    u16::from_str_radix(s, 16).is_ok()
}

pub fn valid_ip_address(query_ip: String) -> String {
    let s = query_ip;
    let ipv6 = s.contains(':');

    let (split, section_num) = if ipv6 {
        (':', 8)
    } else {
        ('.', 4)
    };
    let section_func = if ipv6 { is_ipv6_section } else { is_ipv4_section };
    let bad_result = String::from("Neither");
    let mut cur_section_num = 0;
    for section in s.split(split) {
        cur_section_num += 1;
        if cur_section_num > section_num { return bad_result; }
        if !section_func(section) {
            return String::from("Neither");
        }
    }
    if cur_section_num == section_num {
        return if ipv6 { String::from("IPv6") } else { String::from("IPv4") };
    }
    bad_result
}

fn main() {
    assert_eq!(valid_ip_address(String::from("172.16.254.1")), String::from("IPv4"));
    assert_eq!(valid_ip_address(String::from("172.0.254.1")), String::from("IPv4"));
    assert_eq!(valid_ip_address(String::from("2001:0db8:85a3:0:0:8A2E:0370:7334")), String::from("IPv6"));
    assert_eq!(valid_ip_address(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334")), String::from("IPv6"));
    assert_eq!(valid_ip_address(String::from("02001:0db8:85a3:0000:0000:8a2e:0370:7334")), String::from("Neither"));
    assert_eq!(valid_ip_address(String::from("256.256.256.256")), String::from("Neither"));
    assert_eq!(valid_ip_address(String::from("172..254.1")), String::from("Neither"));
    assert_eq!(valid_ip_address(String::from("172.00.254.1")), String::from("Neither"));
}
