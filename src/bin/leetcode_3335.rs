//! 字符串转换后的长度 I

pub fn length_after_transformations(s: String, t: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    fn dfs(ch: u8, cnt: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if cnt == 0 { return 1; }
        if mem[(ch - b'a') as usize][cnt as usize] != -1 {
            return mem[(ch - b'a') as usize][cnt as usize];
        }
        let result = if ch == b'z' {
            (dfs(b'a', cnt - 1, mem) + dfs(b'b', cnt - 1, mem)) % MOD
        } else {
            dfs(ch + 1, cnt - 1, mem)
        };
        mem[(ch - b'a') as usize][cnt as usize] = result;
        result
    }

    let mut result = 0;
    let mut mem = vec![vec![-1; t as usize + 1]; 26];
    for &ch in s.as_bytes() {
        result = (result + dfs(ch, t, &mut mem)) % MOD;
    }
    result
}

fn main() {
    fn test(func: fn(s: String, t: i32) -> i32) {
        assert_eq!(func(String::from("abcyy"), 2), 7);
        assert_eq!(func(String::from("azbk"), 1), 5);
    }
    test(length_after_transformations);
}
