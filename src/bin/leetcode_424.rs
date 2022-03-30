//! 替换后的最长重复字符

pub fn character_replacement(s: String, k: i32) -> i32 {
    let len = s.len();
    if len == 0 { return 0; }
    let mut dp = [0; 26];
    let s = s.as_bytes();
    let mut ans = 1;
    let mut left = 0;
    let mut right = 0;
    let mut current_max = 0;
    while right < len {
        let idx = (s[right] - b'A') as usize;
        dp[idx] += 1;
        current_max = current_max.max(dp[idx]);
        right += 1;
        if (right - left) as i32 - current_max > k {
            dp[(s[left] - b'A') as usize] -= 1;
            left += 1;
        }
        ans = ans.max(right - left);
    }
    ans as i32
}

fn main() {
    assert_eq!(character_replacement(String::from("ABAB"), 2), 4);
    assert_eq!(character_replacement(String::from("AABABBA"), 1), 4);
}
