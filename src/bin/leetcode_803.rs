//! 打砖块


struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut xx = self.find(x);
        let mut yy = self.find(y);
        if xx == yy { return; }
        if self.size[xx] > self.size[yy] {
            std::mem::swap(&mut xx, &mut yy);
        }
        self.f[xx] = yy;
        self.size[yy] += self.size[xx];
    }

    fn get_size(&mut self, x: usize) -> usize {
        let xx = self.find(x);
        // 以当前结点为根结点的子树的结点总数
        self.size[xx]
    }
}

/// 反向 思考：补上被击碎的砖块以后，有多少个砖块因为这个补上的这个砖块而与屋顶的砖块相连。
pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
    let len = hits.len();
    let m = grid.len();
    let n = grid[0].len();
    let get_idx = |x: usize, y: usize| -> usize {
        x * n + y
    };
    let mut copy = grid.clone();
    let mut result = vec![0; len];
    for hit in &hits {
        copy[hit[0] as usize][hit[1] as usize] = 0;
    }
    let size = m * n;
    let mut us = UnionSet::new(size + 1);
    for j in 0..n {
        if copy[0][j] == 1 {
            us.union(j, size);
        }
    }
    for i in 1..m {
        for j in 0..n {
            if copy[i][j] == 1 {
                if copy[i - 1][j] == 1 {
                    us.union(get_idx(i - 1, j), get_idx(i, j));
                }
                if j > 0 && copy[i][j - 1] == 1 {
                    us.union(get_idx(i, j - 1), get_idx(i, j));
                }
            }
        }
    }
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (i, hit) in hits.iter().enumerate().rev() {
        let x = hit[0] as usize;
        let y = hit[1] as usize;
        // 注意：这里不能用 copy，语义上表示，如果原来在 grid 中，这一块是空白，这一步不会产生任何砖块掉落
        // 逆向补回的时候，与屋顶相连的砖块数量也肯定不会增加
        if grid[x][y] == 0 {
            continue;
        }
        // 补回之前与屋顶相连的砖块数
        let origin = us.get_size(size);
        if x == 0 {
            // 注意：如果补回的这个结点在第 1 行，要告诉并查集它与屋顶相连（逻辑同第 2 步）
            us.union(y, size);
        }
        // 在 4 个方向上看一下，如果相邻的 4 个方向有砖块，合并它们
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if copy[nx][ny] == 1 {
                    us.union(get_idx(x, y), get_idx(nx, ny));
                }
            }
        }
        // 补回之后与屋顶相连的砖块数
        let current = us.get_size(size);
        // 减去的 1 是逆向补回的砖块（正向移除的砖块），与 0 比较大小，是因为存在一种情况，添加当前砖块，不会使得与屋顶连接的砖块数更多
        result[i] = 0.max(current as i32 - origin as i32 - 1);
        // 补上这个砖块
        copy[x][y] = 1;
    }
    result
}

fn main() {
    assert_eq!(hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]], vec![vec![1, 0]]), vec![2]);
    assert_eq!(hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]], vec![vec![1, 1], vec![1, 0]]), vec![0, 0]);
}
