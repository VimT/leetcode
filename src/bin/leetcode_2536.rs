//! 子矩阵元素加 1

/// 二维差分
pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut diff = vec![vec![0; n + 1]; n + 1];
    for query in queries {
        let (x1, y1, x2, y2) = (query[0] as usize, query[1] as usize, query[2] as usize, query[3] as usize);
        diff[x1][y1] += 1;
        diff[x2 + 1][y1] -= 1;
        diff[x1][y2 + 1] -= 1;
        diff[x2 + 1][y2 + 1] += 1;
    }
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = diff[i][j] + if i > 0 { result[i - 1][j] } else { 0 } + if j > 0 { result[i][j - 1] } else { 0 } - if i > 0 && j > 0 { result[i - 1][j - 1] } else { 0 };
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]), vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]);
        assert_eq!(func(2, vec![vec![0, 0, 1, 1]]), vec![vec![1, 1], vec![1, 1]]);
    }
    test(range_add_queries);
}
