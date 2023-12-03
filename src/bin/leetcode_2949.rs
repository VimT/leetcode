//! 统计美丽子字符串 II

use std::collections::HashMap;

/// 子字符串的长度为L, 那么题目要求
/// (L / 2)^2 % k == 0
/// L^2 == 4mk
/// 推论：当k=3时，L必须是3的倍数，k=3^2时，L必须是3的倍数，当k=3^3时，L必须是3^4的倍数。
/// 所以如果k很大，我们需要对k做质因数分解，然后求 L 的约束条件
/// 实际k比较小，我们可以直接枚举约束条件：L=dx，(dx)^2 % 4k == 0，枚举d
/// 所以就是求 presum[i] = presum[j] 且 (j - i) % d == 0 的个数
/// (j - i) % d == 0 等价于 j % d == i % d
/// 综上，我们用哈希表存 (i % d, presum)
pub fn beautiful_substrings(s: String, k: i32) -> i64 {
    let mut k_prime = 0;
    for x in 1.. {
        if x * x % (4 * k) == 0 {
            k_prime = x;
            break;
        }
    }
    let s = s.as_bytes();
    let mut m: HashMap<(i32, i32), i64> = HashMap::new();
    let mut result = 0;
    let mut cur = 0;
    m.insert((0, 0), 1);
    for (&ch, i) in s.iter().zip(1..) {
        if matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u') {
            cur += 1;
        } else {
            cur -= 1;
        }
        result += m.get(&(i % k_prime, cur)).copied().unwrap_or(0);
        *m.entry((i % k_prime, cur)).or_default() += 1;
    }
    result
}


/// 分解质因数做法（实际并没有速度提升）
pub fn beautiful_substrings2(s: String, k: i32) -> i64 {
    fn get_prime(mut x: i32) -> i32 {
        let mut d = 2;
        let mut result = 1;
        while d * d <= x {
            if x % d == 0 {
                let mut multi = true;
                while x % d == 0 {
                    x /= d;
                    if multi { result *= d };
                    multi = !multi;
                }
            }
            d += 1;
        }
        if x > 1 { result *= x; }
        result
    }
    let k_prime = get_prime(k * 4);
    let s = s.as_bytes();
    let mut m: HashMap<(i32, i32), i64> = HashMap::new();
    let mut result = 0;
    let mut cur = 0;
    m.insert((0, 0), 1);
    for (&ch, i) in s.iter().zip(1..) {
        if matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u') {
            cur += 1;
        } else {
            cur -= 1;
        }
        result += m.get(&(i % k_prime, cur)).copied().unwrap_or(0);
        *m.entry((i % k_prime, cur)).or_default() += 1;
    }
    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i64) {
        assert_eq!(func(String::from("baeyh"), 2), 2);
        assert_eq!(func(String::from("abba"), 1), 3);
        assert_eq!(func(String::from("bcdf"), 1), 0);
    }
    test(beautiful_substrings);
    test(beautiful_substrings2);
}
