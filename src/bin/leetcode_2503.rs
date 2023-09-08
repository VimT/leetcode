//! 矩阵查询可获得的最大分数


use std::collections::{BinaryHeap, VecDeque};
use leetcode::union_find::UnionFind;

/// 从左上角bfs，找出到每个点的所有路径中 路径上最大的点 最小的值。
/// 最后query即找 <query的点有多少个。
/// 复杂度高 (368ms)。方法3 优化：遍历排序的queries，边bfs边输出
pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let mut max = vec![vec![i32::MAX; n]; m];
    let mut q = VecDeque::new();
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    q.push_back((0, 0, 0));
    while !q.is_empty() {
        let (i, j, mut cur_max) = q.pop_front().unwrap();
        cur_max = cur_max.max(grid[i][j]);
        if cur_max < max[i][j] {
            max[i][j] = cur_max;
            for (dx, dy) in DIR {
                let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    q.push_back((nx, ny, cur_max));
                }
            }
        }
    }
    let max_num = grid.iter().map(|x| *x.iter().max().unwrap()).max().unwrap() as usize + 1;
    let mut cnt = vec![0; max_num];
    for i in 0..m {
        for j in 0..n {
            cnt[max[i][j] as usize] += 1;
        }
    }
    let mut presum = vec![0; max_num + 1];
    for i in 0..max_num {
        presum[i + 1] = presum[i] + cnt[i];
    }
    queries.into_iter().map(|x| {
        presum[(x as usize).min(max_num)]
    }).collect()
}


/// 并查集
/// 转换成图，把点权转化成边权，边权大小=max(相连的点）。一个query的答案就是左上角的连通图大小。
/// 对queries排序，从小到大依次处理query
pub fn max_points2(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut queries: Vec<(i32, usize)> = queries.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    queries.sort_unstable();
    let m = grid.len();
    let n = grid[1].len();

    // 找出所有边
    let mut edges = Vec::with_capacity(m * n * 2);
    for i in 0..m {
        for j in 0..n {
            if i > 0 { edges.push((grid[i][j].max(grid[i - 1][j]), i * n + j, (i - 1) * n + j)); }
            if j > 0 { edges.push((grid[i][j].max(grid[i][j - 1]), i * n + j, i * n + j - 1)); }
        }
    }
    edges.sort_unstable_by_key(|x| x.0);
    let mut uf = UnionFind::new(m * n);

    let mut result = vec![0; queries.len()];
    let mut j = 0;
    for (query, i) in queries {
        while j < edges.len() && edges[j].0 < query {
            us.union(edges[j].1, edges[j].2);
            j += 1;
        }
        if grid[0][0] < query {
            let i1 = us.find(0);
            result[i] = us.size[i1] as i32;
        }
    }
    result
}

/// 最小堆
pub fn max_points3(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut queries: Vec<(i32, usize)> = queries.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    queries.sort_unstable();
    let m = grid.len();
    let n = grid[1].len();

    let mut q = BinaryHeap::new();
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    q.push((-grid[0][0], 0, 0));
    let mut result = vec![0; queries.len()];

    let mut cnt = 0;
    let mut seen = vec![vec![false; n]; m];
    seen[0][0] = true;
    for (query, i) in queries {
        while !q.is_empty() && -q.peek().unwrap().0 < query {
            let (_, x, y) = q.pop().unwrap();
            cnt += 1;
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !seen[nx][ny] {
                        seen[nx][ny] = true;
                        q.push((-grid[nx][ny], nx, ny));
                    }
                }
            }
        }
        result[i] = cnt;
    }

    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]], vec![5, 6, 2]), vec![5, 8, 1]);
        assert_eq!(func(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]), vec![0]);
    }
    test(max_points);
    test(max_points2);
    test(max_points3);
}
