//! 岛屿数量 II

use leetcode::union_find::UnionFind;

/// 修改版并查集，最开始都指向len（都是水），添加岛屿==isolate隔离，最后看并查集的set数量
pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
    let size = (m * n) as usize;
    let mut uf = UnionFind::new(size);
    us.f.fill(size);
    us.size[size - 1] = size;
    us.count = 0;
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    positions.into_iter().map(|pos| {
        let (x, y) = (pos[0], pos[1]);
        us.isolate((x * n + y) as usize);
        for (dx, dy) in DIR {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < m && ny >= 0 && ny < n {
                if !us.is_empty((nx * n + ny) as usize) {
                    us.union((nx * n + ny) as usize, (x * n + y) as usize);
                }
            }
        }
        us.count as i32
    }).collect()
}

fn main() {
    fn test(func: fn(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(3, 3, vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]]), vec![1, 1, 2, 3]);
        assert_eq!(func(1, 1, vec![vec![0, 0]]), vec![1]);
    }
    test(num_islands2);
}
