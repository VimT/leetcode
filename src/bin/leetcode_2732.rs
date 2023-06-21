//! 找到矩阵中的好子集

/// n<=5决定了，最多只能选两行。如果 n=6，那可能可以选4行
pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid[0].len();
    let size = 1 << n;
    let mut num_map = vec![-1; size];
    for (row, i) in grid.into_iter().zip(0..) {
        let mut num = 0;
        for j in 0..n {
            if row[j] == 1 {
                num |= 1 << j;
            }
        }
        if num == 0 {
            return vec![i];
        }
        for x in 1..size {
            if x & num == 0 && num_map[x] != -1 {
                return vec![num_map[x], i];
            }
        }
        num_map[num] = i;
    }
    vec![]
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 1, 1, 1]]), vec![0, 1]);
        assert_eq!(func(vec![vec![0]]), vec![0]);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 1, 1]]), vec![]);
    }
    test(good_subsetof_binary_matrix);
}
