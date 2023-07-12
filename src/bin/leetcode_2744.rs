//! 最大字符串配对数目

pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut result = 0;
    let mut seen = vec![vec![false; 26]; 26];
    for word in words {
        let w = word.as_bytes();
        let (a, b) = ((w[0] - b'a') as usize, (w[1] - b'a') as usize);
        if seen[b][a] {
            result += 1;
        } else {
            seen[a][b] = true;
        }
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>) -> i32) {
        assert_eq!(func(svec!["cd","ac","dc","ca","zz"]), 2);
        assert_eq!(func(svec!["ab","ba","cc"]), 1);
        assert_eq!(func(svec!["aa","ab"]), 0);
    }
    test(maximum_number_of_string_pairs);
}
