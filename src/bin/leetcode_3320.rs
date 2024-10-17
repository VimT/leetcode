//! 统计能获胜的出招序列数

use std::sync::OnceLock;

pub fn count_winning_sequences(s: String) -> i32 {
    let s: Vec<usize> = s.as_bytes().iter().map(|&ch| match ch {
        b'F' => 0,
        b'W' => 1,
        b'E' => 2,
        _ => unreachable!()
    }).collect();
    let len = s.len();
    const MOD: i32 = 1e9 as i32 + 7;
    static P2: OnceLock<Vec<i32>> = OnceLock::new();
    let pow2 = P2.get_or_init(|| {
        let mut p2 = vec![1; 500];
        for i in 1..500 {
            p2[i] = p2[i - 1] * 2 % MOD;
        }
        p2
    });
    // 当前第 i 局，上一局出了 last，当前分数差为 diff ( 正为 bob 赢）局面下，bob 的出招数量
    fn dfs(s: &Vec<usize>, pow2: &Vec<i32>, i: i32, last: usize, diff: i32, mem: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if -diff > i { return 0; }
        if diff > i + 1 { return pow2[(i + 1) as usize]; }
        if mem[i as usize][last][(diff + (s.len() as i32 + 1) / 2) as usize] != -1 {
            return mem[i as usize][last][(diff + (s.len() as i32 + 1) / 2) as usize];
        }
        let mut result = 0;
        for b in 0..3 {
            if b == last { continue; }
            let mut score = ((b + 3 - s[i as usize]) % 3) as i32;
            if score == 2 { score = -1; }
            result = (result + dfs(s, pow2, i - 1, b, diff + score, mem)) % MOD;
        }
        mem[i as usize][last][(diff + (s.len() as i32 + 1) / 2) as usize] = result;
        result
    }

    let mut mem = vec![vec![vec![-1; s.len() + 2]; 4]; s.len()];
    dfs(&s, pow2, len as i32 - 1, 3, 0, &mut mem)
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("FFW")), 4);
        assert_eq!(func(String::from("FFF")), 3);
        assert_eq!(func(String::from("FWEFW")), 18);
    }
    test(count_winning_sequences);
}
