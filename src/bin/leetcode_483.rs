//! 最小好进制

// sum = k^m + k^(m-1) + k^(m-2) + ... + k^0
pub fn smallest_good_base(n: String) -> String {
    let mut num: u64 = 0;
    for &ch in n.as_bytes() {
        num = num * 10 + (ch - b'0') as u64;
    }
    let m_max = 64 - num.leading_zeros();
    for m in (1..=m_max).rev() {
        let k = (num as f64).powf(1. / m as f64) as u64;
        if k < 2 { continue; }
        let mut mul = 1;
        let mut sum = 1;
        for _ in 0..m {
            mul *= k;
            sum += mul;
        }
        if sum == num {
            return k.to_string();
        }
    }
    (num - 1).to_string()
}

fn main() {
    assert_eq!(smallest_good_base(String::from("3")), String::from("2"));
    assert_eq!(smallest_good_base(String::from("13")), String::from("3"));
    assert_eq!(smallest_good_base(String::from("4681")), String::from("8"));
    assert_eq!(smallest_good_base(String::from("1000000000000000000")), String::from("999999999999999999"));
}
