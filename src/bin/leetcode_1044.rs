//! 最长重复子串

use std::collections::HashSet;

use rand::{Rng, thread_rng};
use leetcode::suffix_array::{cal_suffix_sa, get_height};

#[inline]
fn quick_pow(mut a: i64, mut b: i64, m: i64) -> i64 {
    a = a % m;
    let mut ans = 1;
    while b != 0 {
        if b & 1 == 1 {
            ans = ans * a % m;
            if a < 0 { a += m; }
        }
        a = a * a % m;
        if a < 0 {
            a += m;
        }
        b >>= 1;
    }
    ans
}


pub fn longest_dup_substring(s: String) -> String {
    let s = s.as_bytes();
    let mut sv = [0; 26];
    for &ch in s {
        sv[(ch - b'a') as usize] += 1;
    }
    let mut ok = false;
    for i in 0..26 {
        if sv[i] > 1 {
            ok = true;
            break;
        }
    }
    if !ok { return String::from(""); }
    let len = s.len();
    let mut left = 1;
    let mut right = len;
    let mut last = (0, 0);
    let mut rng = thread_rng();
    let base1 = rng.gen_range(26, 75);
    let base2 = rng.gen_range(26, 75);
    let mod1 = rng.gen_range(1e9 as i64 + 6, i32::MAX as i64);
    let mod2 = rng.gen_range(1e9 as i64 + 6, i32::MAX as i64);
    while left < right {
        let mid = (left + right) / 2;
        let mut set = HashSet::new();
        let mut has = false;
        let mut hash1 = 0;
        let mut hash2 = 0;
        let multi1 = quick_pow(base1, mid as i64, mod1);
        let multi2 = quick_pow(base2, mid as i64, mod2);
        for i in 0..mid {
            hash1 = (hash1 * base1 % mod1 + (s[i] - b'a') as i64) % mod1;
            hash2 = (hash2 * base2 % mod2 + (s[i] - b'a') as i64) % mod2;
        }
        set.insert(hash1);
        for i in mid..len {
            hash1 = (hash1 * base1 % mod1 - (multi1 * (s[i - mid] - b'a') as i64) % mod1 + (s[i] - b'a') as i64) % mod1;
            hash2 = (hash2 * base2 % mod2 - (multi2 * (s[i - mid] - b'a') as i64) % mod2 + (s[i] - b'a') as i64) % mod2;
            if hash1 < 0 { hash1 += mod1; }
            if set.contains(&(hash1)) {
                has = true;
                last = (i + 1 - mid, i);
                break;
            }
            set.insert(hash1);
        }
        if has {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    unsafe { String::from_utf8_unchecked(s[last.0..=last.1].to_vec()) }
}


pub fn longest_dup_substring_sa(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let (rank, sa) = cal_suffix_sa(s);
    let height = get_height(s, &rank, &sa);
    let mut p = 0;
    let mut ll = 0;
    for i in 0..len {
        if height[i] > ll {
            ll = height[i];
            p = sa[i];
        }
    }
    unsafe { String::from_utf8_unchecked(s[p..p + ll].to_vec()) }
}


fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("banana")), String::from("ana"));
        assert_eq!(func(String::from("nnpxouomcofdjuujloanjimymadkuepightrfodmauhrsy")), String::from("ma"));
        assert_eq!(func(String::from("aaaaa")), String::from("aaaa"));
        assert_eq!(func(String::from("abcd")), String::from(""));
    }
    test(longest_dup_substring);
    test(longest_dup_substring_sa);
}
