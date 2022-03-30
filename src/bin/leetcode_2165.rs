//! 重排数字的最小值

pub fn smallest_number(num: i64) -> i64 {
    let mut n = num;
    let neg = n < 0;
    let mut s = vec![];
    while n != 0 {
        s.push((n % 10).abs());
        n /= 10;
    }
    s.sort_unstable();
    if neg {
        let mut num = 0;
        for i in s.into_iter().rev() {
            num = num * 10 + i;
        }
        -num
    } else {
        let mut num = 0;
        let mut i = 0;
        while i < s.len() && s[i] == 0 { i += 1; }
        if i < s.len() {
            num = s[i];
        }
        for _ in 0..i {
            num *= 10;
        }
        for &pos in &s[(i + 1).min(s.len())..] {
            num = num * 10 + pos;
        }
        num
    }
}

fn main() {
    assert_eq!(smallest_number(0), 0);
    assert_eq!(smallest_number(310), 103);
    assert_eq!(smallest_number(-7605), -7650);
}
