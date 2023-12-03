//! 最长相邻不相等子序列 II

pub fn get_words_in_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    fn ok(a: &String, b: &String) -> bool {
        a.len() == b.len() && a.bytes().zip(b.bytes()).filter(|(a, b)| a != b).count() == 1
    }
    let len = n as usize;
    let mut dp = vec![1; len];
    let mut from = vec![len; len];
    let mut mx = len - 1;
    for i in (0..len).rev() {
        for j in i + 1..len {
            if groups[i] != groups[j] && ok(&words[i], &words[j]) {
                if dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    from[i] = j;
                }
            }
        }
        if dp[i] > dp[mx] {
            mx = i;
        }
    }
    let mut result = vec![];
    let mut i = mx;
    while i < len {
        result.push(words[i].clone());
        i = from[i];
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String>) {
        assert_eq!(func(9, svec!["bad","dc","bc","ccd","dd","da","cad","dba","aba"], vec![9,7,1,2,6,8,3,7,2]), vec!["dc","dd","da"]);
        assert_eq!(func(3, svec!["bab","dab","cab"], vec![1, 2, 2]), vec!["bab", "dab"]);
        assert_eq!(func(4, svec!["a","b","c","d"], vec![1, 2, 3, 4]), vec!["a", "b", "c", "d"]);
    }
    test(get_words_in_longest_subsequence);
}
