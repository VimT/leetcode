//! 尽可能使字符串相等

pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let len = s.len();
    let mut diff = vec![0; len];
    for i in 0..len {
        diff[i] = (s[i] as i32 - t[i] as i32).abs()
    }
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let mut cur = 0;
    while right < len {
        if cur + diff[right] > max_cost {
            if left < right {
                cur -= diff[left];
                left += 1;
            } else {
                left += 1;
                right += 1;
            }
        } else {
            cur += diff[right];
            right += 1;
            ans = ans.max(right - left);
        }
    }
    ans as i32
}

fn main() {
    assert_eq!(equal_substring(String::from("abcd"), String::from("bcdf"), 3), 3);
    assert_eq!(equal_substring(String::from("abcd"), String::from("cdef"), 3), 1);
    assert_eq!(equal_substring(String::from("abcd"), String::from("acde"), 0), 1);
}
