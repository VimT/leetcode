//! 统计前后缀下标对 I

pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let len = words.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>) -> i32) {
        assert_eq!(func(svec!["a","aba","ababa","aa"]), 4);
        assert_eq!(func(svec!["pa","papa","ma","mama"]), 2);
        assert_eq!(func(svec!["abab","ab"]), 0);
    }
    test(count_prefix_suffix_pairs);
}
