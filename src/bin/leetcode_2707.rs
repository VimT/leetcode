//! 字符串中的额外字符


use std::collections::HashSet;

pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let dict: HashSet<String> = dictionary.into_iter().collect();
    let len = s.len();
    let mut dp = vec![0; len + 1];
    for i in 0..len {
        dp[i + 1] = dp[i] + 1;
        for j in 0..=i {
            if dict.contains(&s[j..=i]) {
                dp[i + 1] = dp[i + 1].min(dp[j]);
            }
        }
    }
    dp[len]
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(s: String, dictionary: Vec<String>) -> i32) {
        assert_eq!(func(String::from("octncmdbgnxapjoqlofuzypthlytkmchayflwky"), svec!["m","its","imaby","pa","ijmnvj","k","mhka","n","y","nc","wq","p","mjqqa","ht","dfoa","yqa","kk","pixq","ixsdln","rh","dwl","dbgnxa","kmpfz","nhxjm","wg","wky","oct","og","uhin","zxb","qz","tpf","hlrc","j","l","tew","xbn","a","uzypt","uvln","mchay","onnbi","hlytk","pjoqlo","dxsjr","u","uj"]), 2);
        assert_eq!(func(String::from("leetscode"), svec!["leet","code","leetcode"]), 1);
        assert_eq!(func(String::from("sayhelloworld"), svec!["hello","world"]), 3);
    }
    test(min_extra_char);
}
