//! 最长理想子序列

pub fn longest_ideal_string(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut dp = [0; 26];
    for &ch in s {
        for pre in (ch - k as u8).max(b'a')..=(ch + k as u8).min(b'z') {
            dp[(ch - b'a') as usize] = dp[(ch - b'a') as usize].max(dp[(pre - b'a') as usize]);
        }
        dp[(ch - b'a') as usize] += 1;
    }
    *dp.iter().max().unwrap()
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("dyyonfvzsretqxucmavxegvlnnjubqnwrhwikmnnrpzdovjxqdsatwzpdjdsneyqvtvorlwbkb"), 7), 42);
        assert_eq!(func(String::from("lkpkxcigcs"), 6), 7);
        assert_eq!(func(String::from("pvjcci"), 4), 2);
        assert_eq!(func(String::from("acfgbd"), 2), 4);
        assert_eq!(func(String::from("abcd"), 3), 4);
    }
    test(longest_ideal_string);
}
