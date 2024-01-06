//! 并行课程 II


use std::collections::VecDeque;

pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = n as usize;
    let mut pre = vec![0; n];
    for rel in relations {
        pre[rel[1] as usize - 1] |= 1 << (rel[0] - 1);
    }
    struct DFS {
        pre: Vec<usize>,
        n: usize,
        k: i32,
        cache: Vec<i32>,
    }

    impl DFS {
        fn dfs(&mut self, state: usize) -> i32 {
            if state == (1 << self.n) - 1 { return 0; }
            if self.cache[state] != -1 { return self.cache[state]; }
            let mut result = self.n as i32;
            let mut g: usize = 0;
            for i in 0..self.n {
                if state >> i & 1 == 0 && state & self.pre[i] == self.pre[i] {
                    g |= 1 << i;
                }
            }
            // 枚举子集
            let mut s = g;
            while s > 0 {
                if s.count_ones() as i32 <= self.k {
                    result = result.min(1 + self.dfs(state | s));
                }
                s = (s - 1) & g;
            }
            self.cache[state] = result;
            result
        }
    }

    let mut dfs = DFS { pre, n, k, cache: vec![-1; 1 << n] };
    dfs.dfs(0)
}


pub fn min_number_of_semesters2(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut pre = vec![0; n];
    for rel in relations {
        pre[rel[1] as usize - 1] |= 1 << (rel[0] - 1);
    }
    let mut dp = vec![n as i32; 1 << n];
    dp[0] = 0;
    let mut cnt = vec![0; 1 << n];
    for i in 0..1 << n {
        cnt[i] = cnt[i >> 1] + (i & 1);
    }
    let floor = (n / k) as i32 + (n % k != 0) as i32;
    for state in 0..1 << n {
        let mut g: usize = 0;
        for i in 0..n {
            if state >> i & 1 == 0 && state & pre[i] == pre[i] {
                g |= 1 << i;
            }
        }
        let mut s = g;
        while s > 0 {
            if cnt[s] <= k { // cnt 比 s.count_ones() 快40ms
                dp[state | s] = dp[state | s].min(1 + dp[state]);
                if dp[state | s] == floor { break; }  // 剪枝
            }
            s = (s - 1) & g;
        }
    }
    *dp.last().unwrap()
}

/// 性能优化，不用全部遍历 s 的子集，从 s 中组合选 k.min(s.count_ones()) 个，即每次贪心选满
pub fn min_number_of_semesters3(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
    struct Help {
        q: VecDeque<usize>,
        dp: Vec<i32>,
        pre: Vec<usize>,
        can_use: Vec<usize>,
        k: usize,
        u: i32,
    }

    impl Help {
        fn combination(&mut self, c: usize, i: usize, state: usize) {
            if c == self.k || i == self.can_use.len() {
                if self.dp[state] == -1 {
                    self.dp[state] = self.u + 1;
                    self.q.push_back(state);
                }
            } else {
                if self.can_use.len() - i > self.k - c { self.combination(c, i + 1, state); }
                self.combination(c + 1, i + 1, state | 1 << self.can_use[i]);
            }
        }
        fn run(&mut self) -> i32 {
            self.q.push_back(0);
            while !self.q.is_empty() {
                let state = self.q.pop_front().unwrap();
                self.u = self.dp[state];
                self.can_use = (0..self.pre.len()).filter(|&i| state >> i & 1 == 0 && state & self.pre[i] == self.pre[i]).collect();
                self.combination(0, 0, state);
            }
            *self.dp.last().unwrap()
        }
    }
    let n = n as usize;
    let mut pre = vec![0; n];
    for rel in relations {
        pre[rel[1] as usize - 1] |= 1 << (rel[0] - 1);
    }
    let mut dp = vec![-1; 1 << n];
    dp[0] = 0;
    let mut help = Help { q: VecDeque::new(), dp, pre, can_use: vec![], k: k as usize, u: 0 };
    help.run()
}

fn main() {
    fn test(func: fn(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(11, vec![], 2), 6);
        assert_eq!(func(4, vec![vec![2, 1], vec![3, 1], vec![1, 4]], 2), 3);
        assert_eq!(func(5, vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]], 2), 4);
    }
    test(min_number_of_semesters);
    test(min_number_of_semesters2);
    test(min_number_of_semesters3);
}
