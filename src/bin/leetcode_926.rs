//! 将字符串翻转到单调递增

pub fn min_flips_mono_incr(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut pre1 = vec![0; len + 1];
    let mut suf0 = vec![0; len + 1];
    for i in 0..len {
        pre1[i + 1] = pre1[i] + (s[i] == b'1') as i32;
    }
    for i in (0..len).rev() {
        suf0[i] = suf0[i + 1] + (s[i] == b'0') as i32;
    }
    let mut result = i32::MAX;
    for i in 0..=len {
        result = result.min(pre1[i] + suf0[i])
    }
    result
}

fn main() {
    assert_eq!(min_flips_mono_incr(String::from("00110")), 1);
    assert_eq!(min_flips_mono_incr(String::from("010110")), 2);
    assert_eq!(min_flips_mono_incr(String::from("00011000")), 2);
}
