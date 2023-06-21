//! 统计整数数目

/// 数位DP
/// 时间优化关键： mem 不需要存 min_limit 和 max_limit 两个状态，当 这两个为true的时候，状态很少，没有记忆化的必要
pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;

    struct DFS {
        n1: Vec<u8>,
        n2: Vec<u8>,
        min_sum: i32,
        max_sum: i32,
        mem: Vec<Vec<i32>>,
    }

    impl DFS {
        fn dfs(&mut self, i: usize, sum: i32, min_limit: bool, max_limit: bool) -> i32 {
            if sum > self.max_sum { return 0; }
            if i == self.n2.len() {
                return (sum >= self.min_sum && sum <= self.max_sum) as i32;
            }
            if !min_limit && !max_limit && self.mem[i][sum as usize] != -1 {
                return self.mem[i][sum as usize];
            }
            let start = if min_limit { self.n1[i] } else { b'0' };
            let end = if max_limit { self.n2[i] } else { b'9' };
            let mut result = 0;
            for ch in start..=end {
                result += self.dfs(i + 1, sum + (ch - b'0') as i32, min_limit && ch == self.n1[i], max_limit && ch == self.n2[i]) as i64;
            }
            result %= MOD;
            if !min_limit && !max_limit {
                self.mem[i][sum as usize] = result as i32;
            }
            result as i32
        }
    }

    let mut n1 = num1.into_bytes();
    let n2 = num2.into_bytes();
    let len = n2.len();
    while n1.len() < len {
        n1.insert(0, b'0');
    }
    let mut dfs = DFS { n1, n2, min_sum, max_sum, mem: vec![vec![-1; max_sum as usize + 1]; len] };
    dfs.dfs(0, 0, true, true) as i32
}


fn main() {
    fn test(func: fn(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32) {
        assert_eq!(func(String::from("1"), String::from("12"), 1, 8), 11);
        assert_eq!(func(String::from("1"), String::from("5"), 1, 5), 5);
    }
    test(count);
}
