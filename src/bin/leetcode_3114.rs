//! 替换字符可以得到的最晚时间

pub fn find_latest_time(s: String) -> String {
    let mut s = s.into_bytes();
    if s[0] == b'?' {
        s[0] = if s[1] == b'?' || s[1] < b'2' { b'1' } else { b'0' };
    }
    if s[1] == b'?' {
        s[1] = if s[0] == b'?' || s[0] == b'1' { b'1' } else { b'9' };
    }
    if s[3] == b'?' {
        s[3] = b'5';
    }
    if s[4] == b'?' {
        s[4] = b'9';
    }
    String::from_utf8(s).unwrap()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("1?:?4")), String::from("11:54"));
        assert_eq!(func(String::from("0?:5?")), String::from("09:59"));
    }
    test(find_latest_time);
}
