//! 角矩形的数量

pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid[0].len();
    let mut count = vec![vec![0; n]; n];
    let mut result = 0;
    for row in grid {
        for j1 in 0..n {
            if row[j1] == 1 {
                for j2 in j1 + 1..n {
                    if row[j2] == 1 {
                        result += count[j1][j2];
                        count[j1][j2] += 1;
                    }
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 0, 1, 0], vec![0, 0, 1, 0, 1], vec![0, 0, 0, 1, 0], vec![1, 0, 1, 0, 1]]), 1);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]), 9);
        assert_eq!(func(vec![vec![1, 1, 1, 1]]), 0);
    }
    test(count_corner_rectangles);
}
