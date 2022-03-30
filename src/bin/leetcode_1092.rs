//! 最短公共超序列


pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let mut before = vec![vec![(0, 0); s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                before[i][j] = (i - 1, j - 1);
            }
            if dp[i - 1][j] >= dp[i][j] {
                dp[i][j] = dp[i - 1][j];
                before[i][j] = (i - 1, j);
            }
            if dp[i][j - 1] >= dp[i][j] {
                dp[i][j] = dp[i][j - 1];
                before[i][j] = (i, j - 1);
            }
        }
    }
    let mut result = Vec::with_capacity(s1.len() + s2.len());
    let (mut i, mut j) = (s1.len(), s2.len());
    while i > 0 && j > 0 {
        let (ii, jj) = before[i][j];
        if ii != i && jj != j {
            result.push(s1[i - 1]);
        } else if ii != i {
            result.push(s1[i - 1]);
        } else if jj != j {
            result.push(s2[j - 1]);
        }
        i = ii;
        j = jj;
    }
    while i > 0 {
        result.push(s1[i - 1]);
        i -= 1;
    }
    while j > 0 {
        result.push(s2[j - 1]);
        j -= 1;
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(str1: String, str2: String) -> String) {
        assert_eq!(func(String::from("acbbcccaa"), String::from("bbbcaaaaa")), "acbbbcccaaaaa");
        assert_eq!(func(String::from("bcacaaab"), String::from("bbabaccc")), "bcbacaaabaccc");
        assert_eq!(func(String::from("abac"), String::from("cab")), "cabac");
        assert_eq!(func(String::from("aaaaaaaa"), String::from("aaaaaaaa")), "aaaaaaaa");
    }
    test(shortest_common_supersequence);
}
