//! 地图中的最高点

/// 多源BFS
pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();
    let mut q = vec![];
    let mut result = vec![vec![-1; n]; m];
    for i in 0..m {
        for j in 0..n {
            if is_water[i][j] == 1 {
                q.push((i, j));
                result[i][j] = 0;
            }
        }
    }
    let mut height = 0;
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while !q.is_empty() {
        let mut nq = Vec::with_capacity(q.len() * 4);
        for (x, y) in q {
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if result[nx][ny] == -1 {
                        result[nx][ny] = height + 1;
                        nq.push((nx, ny));
                    }
                }
            }
        }
        height += 1;
        q = nq;
    }
    result
}

fn main() {
    assert_eq!(highest_peak(vec![vec![0, 1], vec![0, 0]]), vec![vec![1, 0], vec![2, 1]]);
    assert_eq!(highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]), vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]);
}
