//! 按距离统计房屋对数目 I

pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![i32::MAX / 2; n]; n];
    for i in 0..n - 1 {
        g[i][i + 1] = 1;
        g[i + 1][i] = 1;
    }
    g[x as usize - 1][y as usize - 1] = 1;
    g[y as usize - 1][x as usize - 1] = 1;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }
    let mut result = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                result[g[i][j] as usize - 1] += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, x: i32, y: i32) -> Vec<i32>) {
        assert_eq!(func(3, 1, 3), vec![6, 0, 0]);
        assert_eq!(func(5, 2, 4), vec![10, 8, 2, 0, 0]);
        assert_eq!(func(4, 1, 1), vec![6, 4, 2, 0]);
    }
    test(count_of_pairs);
}
