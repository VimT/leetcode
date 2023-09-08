//! 范围中美丽整数的数目

/// 数位DP
pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
    struct DFS {
        n1: Vec<i32>,
        n2: Vec<i32>,
        k: i32,
        mem: Vec<Vec<Vec<i32>>>,
    }

    impl DFS {
        /// diff: 奇数位与偶数位的差, val: 当前数 % k，is_num: 是有效数？ （is_num 并不影响 dp 结果，所以不参与mem计算）
        fn dfs(&mut self, i: usize, val: i32, diff: usize, is_num: bool, min_limit: bool, max_limit: bool) -> i32 {
            if i == self.n2.len() {
                return (val == 0 && diff == self.n2.len()) as i32;
            }
            if !min_limit && !max_limit && self.mem[i][val as usize][diff] != -1 {
                return self.mem[i][val as usize][diff];
            }
            let start = if min_limit { self.n1[i] } else { 0 };
            let end = if max_limit { self.n2[i] } else { 9 };
            let mut result = 0;
            for ch in start..=end {
                let is_num = is_num || ch > 0;
                result += self.dfs(i + 1, (val * 10 + ch) % self.k, if is_num { diff + ch as usize % 2 * 2 - 1 } else { diff }, is_num, min_limit && ch == self.n1[i], max_limit && ch == self.n2[i]) as i64;
            }
            if !min_limit && !max_limit {
                self.mem[i][val as usize][diff] = result as i32;
            }
            result as i32
        }
    }

    let mut n1: Vec<i32> = low.to_string().as_bytes().iter().map(|x| (*x - b'0') as i32).collect();
    let n2: Vec<i32> = high.to_string().as_bytes().iter().map(|x| (*x - b'0') as i32).collect();
    let len = n2.len();
    while n1.len() < len {
        n1.insert(0, 0);
    }
    let mut dfs = DFS { n1, n2, k, mem: vec![vec![vec![-1; 20]; k as usize]; len] };
    dfs.dfs(0, 0, len, false, true, true)
}

fn main() {
    fn test(func: fn(low: i32, high: i32, k: i32) -> i32) {
        assert_eq!(func(1, 10, 1), 1);
        assert_eq!(func(10, 20, 3), 2);
        assert_eq!(func(5, 5, 2), 0);
    }
    test(number_of_beautiful_integers);
}
