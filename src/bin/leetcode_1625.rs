//! 执行操作后字典序最小的字符串

use leetcode::gcd::gcd;

pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let len = s.len();
    let s = s.repeat(2).into_bytes();
    // 初始pos=0,每次(pos+b)%len
    // 用表达式表示最终到达的位置：xb-yn=z，x是轮转次数，y是减去n的次数，z是最终到达的位置
    // 根据裴蜀定理可知，z一定是gcd(b,n)的倍数，所以只需要枚举 [0,n)中 gcd(b,n)的倍数即可。
    let g = gcd(len as i32, b as i32) as usize;
    let mut i = 0;
    let mut result = s[..len].to_vec();
    let a = a as u8;
    while i < len {
        for add1 in 0..=9 {
            let max_add2 = if g & 1 == 0 { 0 } else { 9 };
            for add2 in 0..=max_add2 {
                let mut p = s[i..i + len].to_vec();
                let mut j = 1;
                while j < len {
                    p[j] = (p[j] - b'0' + add1 * a) % 10 + b'0';
                    j += 2;
                }
                j = 0;
                while j < len {
                    p[j] = (p[j] - b'0' + add2 * a) % 10 + b'0';
                    j += 2;
                }
                if p < result {
                    result = p;
                }
            }
        }
        i += g;
    }
    unsafe { String::from_utf8_unchecked(result) }
}


/// 时间复杂度优化
pub fn find_lex_smallest_string2(s: String, a: i32, b: i32) -> String {
    let len = s.len();
    let mut result = s.clone().into_bytes();
    let s = s.repeat(2).into_bytes();
    let ga = gcd(10, a) as u8;
    let gb = gcd(len as i32, b) as usize;

    // 让一个位置经过a次操作变成最小
    let add = |p: &mut [u8], pos: usize| {
        let mut lo = p[pos] - b'0';
        let mut added = 0;
        let mut i = ga;
        while i < 10 {
            let c = (p[pos] - b'0' + i) % 10;
            if c < lo {
                lo = c;
                added = i;
            }
            i += ga;
        }
        if added > 0 {
            let mut i = pos;
            while i < len {
                p[i] = (p[i] - b'0' + added) % 10 + b'0';
                i += 2;
            }
        }
    };

    // 旋转
    let mut i = 0;
    while i < len {
        let mut p = s[i..i + len].to_vec();
        add(&mut p, 1);
        if gb & 1 == 1 {
            add(&mut p, 0);
        }
        if p < result {
            result = p;
        }
        i += gb;
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String, a: i32, b: i32) -> String) {
        assert_eq!(func(String::from("247303203266"), 3, 1), String::from("000273963217"));
        assert_eq!(func(String::from("5525"), 9, 2), String::from("2050"));
        assert_eq!(func(String::from("74"), 5, 1), String::from("24"));
        assert_eq!(func(String::from("0011"), 4, 2), String::from("0011"));
    }
    test(find_lex_smallest_string);
    test(find_lex_smallest_string2);
}
