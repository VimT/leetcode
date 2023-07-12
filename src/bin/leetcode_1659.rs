//! 最大化网格幸福感

/// 轮廓线DP
/// 只有前 n 个格子可能与当前格子发生“相互作用”
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let (a, b) = (introverts_count as usize, extroverts_count as usize);
    // 只保存一行的状态
    let status_len = 3usize.pow(n as u32);
    let status_len1 = status_len / 3;
    let mut dp = vec![vec![vec![vec![0; status_len]; b + 1]; a + 1]; n * m + 1];
    let offset = [[0, 0, 0], [0, -60, -10], [0, -10, 40]];
    for c in (0..m * n).rev() {
        let j = c % n;
        for x in 0..=a {
            for y in 0..=b {
                for pre in 0..status_len { // pre 就是前 n 个格子的状态（三进制）
                    let nem = (pre * 3) % status_len; // nem 是 pre “左移” 一位, 并去掉首位的状态,比如三进制 2121->三进制 1210.
                    if x > 0 {
                        let diff = 120 + if j != 0 { offset[1][pre % 3] } else { 0 } + offset[1][pre / status_len1];
                        dp[c][x][y][pre] = dp[c][x][y][pre].max(diff + dp[c + 1][x - 1][y][nem + 1]);
                    }
                    if y > 0 {
                        let diff = 40 + if j != 0 { offset[2][pre % 3] } else { 0 } + offset[2][pre / status_len1];
                        dp[c][x][y][pre] = dp[c][x][y][pre].max(diff + dp[c + 1][x][y - 1][nem + 2]);
                    }
                    dp[c][x][y][pre] = dp[c][x][y][pre].max(dp[c + 1][x][y][nem]);
                }
            }
        }
    }
    dp[0][a][b][0]
}

/// 状态压缩
pub fn get_max_grid_happiness2(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    struct DFS {
        // ix[cur] 表示当前行状态 cur 中内向的人个数
        ix: Vec<usize>,
        // ex[cur] 表示当前行状态 cur 中外向的人个数
        ex: Vec<usize>,
        // f[cur] 表示状态 cur 的人的初始幸福感
        f: Vec<i32>,
        // g[pre][cur] 表示相邻两个状态行对幸福感的贡献
        g: Vec<Vec<i32>>,
        mem: Vec<Vec<Vec<Vec<i32>>>>,
        m: usize,
        status: usize,
    }

    impl DFS {
        // 表示当前第 i 行，且上一行的状态为 pre，内向的人还剩 ic 个，外向的人还剩 ec 个时，网格的最大幸福感。
        fn dfs(&mut self, i: usize, pre: usize, ic: usize, ec: usize) -> i32 {
            if i == self.m || (ic == 0 && ec == 0) { return 0; }
            if self.mem[i][pre][ic][ec] != -1 {
                return self.mem[i][pre][ic][ec];
            }
            let mut result = 0;
            for cur in 0..self.status {
                if ic >= self.ix[cur] && ec >= self.ex[cur] {
                    result = result.max(self.f[cur] + self.g[pre][cur] + self.dfs(i + 1, cur, ic - self.ix[cur], ec - self.ex[cur]));
                }
            }
            self.mem[i][pre][ic][ec] = result;
            result
        }
    }

    let m = m as usize;
    let n = n as usize;
    let status = 3usize.pow(n as u32);
    let ic = introverts_count as usize;
    let ec = extroverts_count as usize;
    let mem = vec![vec![vec![vec![-1; ec + 1]; ic + 1]; status]; m];
    let mut ix = vec![0; status];
    let mut ex = vec![0; status];
    let mut f = vec![0; status];
    let mut g = vec![vec![0; status]; status];
    let mut bits = vec![vec![0; n]; status];
    static H: [[i32; 3]; 3] = [[0, 0, 0], [0, -60, -10], [0, -10, 40]];
    for i in 0..status {
        let mut mask = i;
        let mut ic = 0;
        let mut ec = 0;
        let mut x = 0;
        for j in 0..n {
            let idx = mask % 3;
            bits[i][j] = idx;
            match idx {
                1 => {
                    ic += 1;
                    x += 120;
                }
                2 => {
                    ec += 1;
                    x += 40;
                }
                _ => (),
            }
            mask /= 3;
            if j > 0 {
                x += H[idx][bits[i][j - 1]];
            }
        }
        ix[i] = ic;
        ex[i] = ec;
        f[i] = x;
    }
    for i in 0..status {
        for j in 0..status {
            for k in 0..n {
                g[i][j] += H[bits[i][k]][bits[j][k]];
            }
        }
    }
    let mut dfs = DFS { ix, ex, f, g, mem, m, status };
    dfs.dfs(0, 0, ic, ec)
}

/// 轮廓线DP
pub fn get_max_grid_happiness3(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    static H: [[i32; 3]; 3] = [[0, 0, 0], [0, -60, -10], [0, -10, 40]];
    struct DFS {
        m: usize,
        n: usize,
        status1: usize,
        mem: Vec<Vec<Vec<Vec<i32>>>>,
    }
    impl DFS {
        // 当前搜索到位置pos，且此前的n个网格单元的状态为pre，内向的人还剩ic个，外向的人还剩ec个时，网格的最大幸福感。
        fn dfs(&mut self, pos: usize, pre: usize, ic: usize, ec: usize) -> i32 {
            if pos == self.m * self.n || (ic == 0 && ec == 0) { return 0; }
            if self.mem[pos][pre][ic][ec] != -1 {
                return self.mem[pos][pre][ic][ec];
            }
            let up = pre / self.status1;
            let left = if pos % self.n == 0 { 0 } else { pre % 3 };
            let mut result = 0;
            for i in 0..3 {
                if (i == 1 && ic == 0) || (i == 2 && ec == 0) { continue; }
                let cur = pre % self.status1 * 3 + i;
                let a = H[up][i] + H[left][i];
                let b = self.dfs(pos + 1, cur, ic - (i == 1) as usize, ec - (i == 2) as usize);
                let c = if i == 1 { 120 } else if i == 2 { 40 } else { 0 };
                result = result.max(a + b + c);
            }
            self.mem[pos][pre][ic][ec] = result;
            result
        }
    }
    let ec = extroverts_count as usize;
    let ic = introverts_count as usize;
    let mut dfs = DFS {
        m: m as usize,
        n: n as usize,
        status1: 3usize.pow((n - 1) as u32),
        mem: vec![vec![vec![vec![-1; ec + 1]; ic + 1]; 3usize.pow(n as u32)]; (m * n) as usize],
    };
    dfs.dfs(0, 0, ic, ec)
}

fn main() {
    fn test(func: fn(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32) {
        assert_eq!(func(2, 3, 1, 2), 240);
        assert_eq!(func(3, 1, 2, 1), 260);
        assert_eq!(func(2, 2, 4, 0), 240);
    }
    test(get_max_grid_happiness);
    test(get_max_grid_happiness2);
    test(get_max_grid_happiness3);
}
