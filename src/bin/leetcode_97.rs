//! 交错字符串


/// 暴力
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() { return false; }

    fn inner(s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
        if s3.len() == 0 { return true; }
        if s1.len() > 0 {
            if s1[0] == s3[0] {
                if inner(&s1[1..], s2, &s3[1..]) { return true; }
            }
        }
        if s2.len() > 0 {
            if s2[0] == s3[0] {
                if inner(s1, &s2[1..], &s3[1..]) { return true; }
            }
        }
        false
    }

    inner(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
}

/// 记忆化暴力
pub fn is_interleave_mem(s1: String, s2: String, s3: String) -> bool {
    fn inner(s1: &[u8], i: usize, s2: &[u8], j: usize, s3: &[u8], k: usize, mem: &mut Vec<Vec<i8>>) -> bool {
        if i == s1.len() { return s2[j..] == s3[k..]; }
        if j == s2.len() { return s1[i..] == s3[k..]; }
        if mem[i][j] >= 0 { return mem[i][j] != 0; }
        let ans = (s1[i] == s3[k] && inner(s1, i + 1, s2, j, s3, k + 1, mem)) ||
            (s2[j] == s3[k] && inner(s1, i, s2, j + 1, s3, k + 1, mem));
        mem[i][j] = ans as i8;
        ans
    }
    inner(s1.as_bytes(), 0, s2.as_bytes(), 0, s3.as_bytes(), 0, &mut vec![vec![-1; s2.len()]; s1.len()])
}

/// dp[i][j]  s1[..i] 和 s2[..j] 能否组成 s3[..i+j]
pub fn is_interleave_dp(s1: String, s2: String, s3: String) -> bool {
    let s3 = s3.as_bytes();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let m = s1.len();
    let n = s2.len();

    if s3.len() != m + n { return false; }

    let mut dp = vec![vec![false; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 && j == 0 {
                dp[i][j] = true;
            } else if i == 0 {
                dp[i][j] = dp[i][j - 1] && s2[j - 1] == s3[i + j - 1];
            } else if j == 0 {
                dp[i][j] = dp[i - 1][j] && s1[i - 1] == s3[i + j - 1];
            } else {
                let s1_bool = dp[i - 1][j] && s1[i - 1] == s3[i + j - 1];
                let s2_bool = dp[i][j - 1] && s2[j - 1] == s3[i + j - 1];
                dp[i][j] = s1_bool || s2_bool;
            }
        }
    }

    dp[m][n]
}

fn main() {
    fn test(func: fn(s1: String, s2: String, s3: String) -> bool) {
        assert_eq!(func(String::from("aabcc"), String::from("dbbca"), String::from("aadbbcbcac")), true);
        assert_eq!(func(String::from("aabcc"), String::from("dbbca"), String::from("aadbbbaccc")), false);
        assert_eq!(func(String::from(""), String::from(""), String::from("")), true);
    }
    test(is_interleave);
    test(is_interleave_dp);
    test(is_interleave_mem);
}
