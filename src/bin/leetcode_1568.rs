//! 使陆地分离的最少天数

use std::collections::VecDeque;

/// 脑筋急转弯，最大的结果是2。所以先判断是不是已经是分开的。接着枚举把每个陆地变成水路，能否分开。否则结果就是2
pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    let mut cnt = 0;
    let mut land_cnt = 0;
    let mut cur_iter = 1;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == cur_iter {
                cnt += 1;
                grid[i][j] += 1;
                q.push_back((i, j));
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    land_cnt += 1;
                    for (dx, dy) in DIR {
                        let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                        if nx < m && ny < n && grid[nx][ny] == cur_iter {
                            grid[nx][ny] += 1;
                            q.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }
    if cnt != 1 { return 0; }
    if land_cnt == 1 { return 1; }
    cur_iter += 1;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == cur_iter {
                grid[i][j] += 1;
                for (dx, dy) in DIR {
                    let (nx, ny) = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
                    if nx < m && ny < n && grid[nx][ny] == cur_iter {
                        q.push_back((nx, ny));
                        grid[nx][ny] += 1;
                        let mut this_land_cnt = 0;
                        while !q.is_empty() {
                            let (x, y) = q.pop_front().unwrap();
                            this_land_cnt += 1;
                            for (dx, dy) in DIR {
                                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                                if nx < m && ny < n && grid[nx][ny] == cur_iter {
                                    grid[nx][ny] += 1;
                                    q.push_back((nx, ny));
                                }
                            }
                        }
                        if this_land_cnt < land_cnt - 1 {
                            return 1;
                        }
                        break;
                    }
                }
                cur_iter += 1;
            }
        }
    }
    2
}


/// Tarjan 找割点
pub fn min_days2(grid: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    struct Tarjan<'a> {
        edge: &'a Vec<Vec<usize>>,
        // 追溯值：以当前节点作为搜索树的根节点，能访问到的所有节点中，时间戳最小的值
        low: Vec<usize>,
        // 深度优先搜索遍历时结点 u 被搜索的次序（时间戳）
        dfn: Vec<usize>,
        cut: i32,
    }

    impl<'a> Tarjan<'a> {
        fn new(edge: &'a Vec<Vec<usize>>) -> Self {
            let len = edge.len();
            Self { edge, low: vec![0; len], dfn: vec![0; len], cut: 0 }
        }
        fn tarjan(&mut self, u: usize, cnt: usize) {
            self.dfn[u] = cnt;
            self.low[u] = cnt;
            let mut is_cut = false;
            let mut child = 0;
            for &v in &self.edge[u] {
                if self.dfn[v] == 0 {
                    child += 1;
                    self.tarjan(v, cnt + 1);
                    self.low[u] = self.low[u].min(self.low[v]);

                    // 是割点
                    if !is_cut && cnt != 1 && self.low[v] >= self.dfn[u] {
                        is_cut = true;
                        self.cut += 1;
                    }
                } else if self.dfn[v] != self.dfn[u] - 1 {
                    self.low[u] = self.low[u].min(self.dfn[v]);
                }
            }
            if !is_cut && child >= 2 && cnt == 1 {
                self.cut += 1;
            }
        }
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut map = vec![0; m * n];
    let mut cnt = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                map[i * n + j] = cnt;
                cnt += 1;
            }
        }
    }
    if cnt < 2 { return cnt as i32; }
    let mut edges = vec![vec![]; cnt];
    for x in 0..m {
        for y in 0..n {
            if grid[x][y] == 1 {
                for (dx, dy) in DIR {
                    let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                    if nx < m && ny < n && grid[nx][ny] == 1 {
                        edges[map[x * n + y]].push(map[nx * n + ny]);
                    }
                }
            }
        }
    }


    let mut t = Tarjan::new(&edges);
    let mut cc_cnt = 0;
    for i in 0..cnt {
        if t.dfn[i] == 0 {
            cc_cnt += 1;
            t.tarjan(i, 1);
        }
    }
    if cc_cnt > 1 { return 0; }
    return if t.cut == 0 { 2 } else { 1 }
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1,1,0,1,1],vec![1,1,1,1,1],vec![1,1,0,1,1],vec![1,1,0,1,1]]), 1);
        assert_eq!(func(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 1);
        assert_eq!(func(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]), 2);
        assert_eq!(func(vec![vec![1, 1]]), 2);
    }
    test(min_days);
    test(min_days2);
}
