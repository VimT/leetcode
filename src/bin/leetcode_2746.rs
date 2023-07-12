//! 字符串连接删减字母

pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
    fn dfs(words: &Vec<String>, i: usize, s: u8, e: u8, mem: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if i == words.len() { return 0; }
        if mem[i][s as usize][e as usize] != -1 { return mem[i][s as usize][e as usize]; }
        let word = words[i].as_bytes();
        let a = word[0] - b'a';
        let b = word.last().copied().unwrap() - b'a';
        let result = (dfs(words, i + 1, s, b, mem) - (a == e) as i32)
            .min(dfs(words, i + 1, a, e, mem) - (b == s) as i32)
            + word.len() as i32;
        mem[i][s as usize][e as usize] = result;
        return result;
    }
    let w0 = words[0].as_bytes();
    let mut mem = vec![vec![vec![-1; 26]; 26]; words.len()];
    w0.len() as i32 + dfs(&words, 1, w0[0] - b'a', w0.last().copied().unwrap() - b'a', &mut mem)
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>) -> i32) {
        assert_eq!(func(svec!["aa","ab","bc"]), 4);
        assert_eq!(func(svec!["ab","b"]), 2);
        assert_eq!(func(svec!["aaa","c","aba"]), 6);
    }
    test(minimize_concatenated_length);
}
