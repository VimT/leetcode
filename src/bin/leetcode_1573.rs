//! 分割字符串的方案数

pub fn num_ways(s: String) -> i32 {
    let s = s.as_bytes();
    let cnt = s.iter().filter(|x| **x == b'1').count();
    if cnt % 3 != 0 { return 0; }
    if cnt == 0 { return (((s.len() - 2) * (s.len() - 1) / 2) % (1e9 as usize + 7)) as i32; }
    let every = cnt / 3;
    let mut i = 0;
    for _ in 0..every {
        while s[i] != b'1' {
            i += 1;
        }
        i += 1;
    }
    let mut a = 0;
    while s[i] == b'0' {
        i += 1;
        a += 1;
    }
    for _ in 0..every {
        while s[i] != b'1' { i += 1; }
        i += 1;
    }
    let mut b = 0;
    while s[i] == b'0' {
        i += 1;
        b += 1;
    }
    ((a + 1) * (b + 1) % (1e9 as i64 + 7)) as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("10101")), 4);
        assert_eq!(func(String::from("1001")), 0);
        assert_eq!(func(String::from("0000")), 3);
        assert_eq!(func(String::from("000")), 1);
        assert_eq!(func(String::from("100100010100110")), 12);
    }
    test(num_ways);
}
