//! 扰乱字符串

fn is_charts_equal(b1: &[u8], b2: &[u8]) -> bool {
    if b1.len() != b2.len() { return false; }
    let mut cnt = [0; 256];
    for b in b1 {
        cnt[*b as usize] += 1;
    }
    for b in b2 {
        if cnt[*b as usize] == 0 { return false; }
        cnt[*b as usize] -= 1;
    }
    true
}

/// 递归
pub fn is_scramble(s1: String, s2: String) -> bool {
    fn inner(b1: &[u8], b2: &[u8]) -> bool {
        // println!("{},{}", String::from_utf8_lossy(b1), String::from_utf8_lossy(b2));
        let len = b1.len();
        if !is_charts_equal(b1, b2) { return false; }
        if len <= 3 { return true; }
        for i in 1..len {
            // 没交换
            if inner(&b1[0..i], &b2[0..i]) && inner(&b1[i..], &b2[i..]) { return true; }
            // 交换了
            if inner(&b1[0..i], &b2[len - i..]) && inner(&b1[i..], &b2[0..len - i]) { return true; }
        }
        false
    }
    inner(s1.as_bytes(), s2.as_bytes())
}

/// dp[i][j][l] 表示 [i:i+l] 和 [j:j+l] 是否扰乱
pub fn is_scramble_dp(s1: String, s2: String) -> bool {
    let len = s1.len();
    let mut dp = vec![vec![vec![false; len + 1]; len]; len];
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    for i in 0..len {
        for j in 0..len {
            dp[i][j][1] = b1[i] == b2[j];
        }
    }
    // 切片从小到大
    for l in 2..=len {
        for i in 0..=len - l {
            for j in 0..=len - l {
                // 依次判断 划分位置
                for k in 1..l {
                    if dp[i][j][k] && dp[i + k][j + k][l - k] {
                        dp[i][j][l] = true;
                        break;
                    }
                    if dp[i][j + l - k][k] && dp[i + k][j][l - k] {
                        dp[i][j][l] = true;
                        break;
                    }
                }
            }
        }
    }

    dp[0][0][len]
}

fn main() {
    fn test(func: fn(s1: String, s2: String) -> bool) {
        assert_eq!(func(String::from("great"), String::from("rgeat")), true);
        assert_eq!(func(String::from("abcde"), String::from("caebd")), false);
        assert_eq!(func(String::from("a"), String::from("a")), true);
    }
    test(is_scramble);
    test(is_scramble_dp);
}
