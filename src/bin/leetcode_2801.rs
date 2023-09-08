//! 统计范围内的步进数字数目

pub fn count_stepping_numbers(low: String, high: String) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;

    struct DFS {
        n1: Vec<u8>,
        n2: Vec<u8>,
        mem: Vec<Vec<i32>>,
    }

    impl DFS {
        fn dfs(&mut self, i: usize, prev_num: i32, min_limit: bool, max_limit: bool) -> i32 {
            if i == self.n2.len() {
                return 1;
            }
            if prev_num != -1 && !min_limit && !max_limit && self.mem[i][prev_num as usize] != -1 {
                return self.mem[i][prev_num as usize];
            }

            let start = if min_limit { self.n1[i] } else { b'0' };
            let end = if max_limit { self.n2[i] } else { b'9' };
            let mut result = 0;
            if prev_num == -1 {
                for ch in start..=end {
                    result += self.dfs(i + 1, if ch == b'0' { -1 } else { (ch - b'0') as i32 }, min_limit && ch == self.n1[i], max_limit && ch == self.n2[i]) as i64;
                }
            } else {
                let a = prev_num as u8 + b'0' + 1;
                let b = prev_num as u8 + b'0' - 1;
                for ch in [a, b] {
                    if ch >= start && ch <= end {
                        result += self.dfs(i + 1, (ch - b'0') as i32, min_limit && ch == self.n1[i], max_limit && ch == self.n2[i]) as i64;
                    }
                }
            }

            result %= MOD;
            if prev_num != -1 && !min_limit && !max_limit {
                self.mem[i][prev_num as usize] = result as i32;
            }

            result as i32
        }
    }

    let mut n1 = low.into_bytes();
    let n2 = high.into_bytes();
    let len = n2.len();
    while n1.len() < len {
        n1.insert(0, b'0');
    }
    let mut dfs = DFS { n1, n2, mem: vec![vec![-1; 10]; len] };
    dfs.dfs(0, -1, true, true)
}

fn main() {
    fn test(func: fn(low: String, high: String) -> i32) {
        assert_eq!(func(String::from("1"), String::from("11")), 10);
        assert_eq!(func(String::from("90"), String::from("101")), 2);
    }
    test(count_stepping_numbers);
}
