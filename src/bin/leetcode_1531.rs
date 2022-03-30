//! 压缩字符串 II

/// dp[i][cnt] 从第i个字符开始，选择cnt个字符，使编码长度最小
pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let t = s.len() - k as usize;
    let calc = |x: usize| { if x <= 1 { x } else { if x <= 9 { 2 } else { if x <= 99 { 3 } else { 4 } } } };
    let mut dp = vec![vec![9999999; t + 1]; s.len() + 1];
    dp[s.len()][t] = 0;
    for p in (0..s.len()).rev() {
        for cnt in 0..=t {
            let mut same = 0;
            for j in p..s.len() {
                same += (s[j] == s[p]) as usize;
                if same + cnt > t { break; }
                dp[p][cnt] = dp[p][cnt].min(calc(same) as i32 + dp[j + 1][cnt + same]);
            }
            dp[p][cnt] = dp[p][cnt].min(dp[p + 1][cnt]);
        }
    }
    dp[0][0]
}

fn main() {
    assert_eq!(get_length_of_optimal_compression(String::from("aaabcccd"), 2), 4);
    assert_eq!(get_length_of_optimal_compression(String::from("aabbaa"), 2), 2);
    assert_eq!(get_length_of_optimal_compression(String::from("aaaaaaaaaaa"), 0), 3);
}
