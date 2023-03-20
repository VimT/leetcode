//! 改变一个整数能得到的最大差值

pub fn max_diff(num: i32) -> i32 {
    let s = num.to_string();
    let sb = s.as_bytes();
    let mut max = num;
    if let Some(&ch) = sb.iter().find(|&&x| x != b'9') {
        max = s.replace(ch as char, "9").parse().unwrap();
    }
    let mut min = num;
    if sb[0] != b'1' {
        min = s.replace(sb[0] as char, "1").parse().unwrap();
    } else if let Some(&ch) = sb[1..].iter().find(|&&x| x != sb[0] && x != b'0') {
        min = s.replace(ch as char, "0").parse().unwrap();
    }
    max - min
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(555), 888);
        assert_eq!(func(9), 8);
    }
    test(max_diff);
}
