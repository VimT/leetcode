//! 球会落何处

pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    fn dfs(grid: &Vec<Vec<i32>>, cur: (usize, usize), cache: &mut Vec<Vec<i32>>) -> i32 {
        if cur.0 == grid.len() {
            return cur.1 as i32;
        }
        if cache[cur.0][cur.1] != -2 {
            return cache[cur.0][cur.1];
        }
        let result = if grid[cur.0][cur.1] == -1 {
            if cur.1 > 0 && grid[cur.0][cur.1 - 1] == -1 {
                dfs(grid, (cur.0 + 1, cur.1 - 1), cache)
            } else {
                -1
            }
        } else {
            if cur.1 + 1 < grid[0].len() && grid[cur.0][cur.1 + 1] == 1 {
                dfs(grid, (cur.0 + 1, cur.1 + 1), cache)
            } else {
                -1
            }
        };
        cache[cur.0][cur.1] = result;
        result
    }
    let rows = grid.len();
    let columns = grid[0].len();
    let mut result = vec![0; columns];
    let mut cache = vec![vec![-2; columns]; rows];
    for i in 0..columns {
        result[i] = dfs(&grid, (0, i), &mut cache);
    }
    result
}

fn main() {
    assert_eq!(find_ball(vec![vec![1, 1, 1, -1, -1], vec![1, 1, 1, -1, -1], vec![-1, -1, -1, 1, 1], vec![1, 1, 1, 1, -1], vec![-1, -1, -1, -1, -1]]), vec![1, -1, -1, -1, -1]);
    assert_eq!(find_ball(vec![vec![-1]]), vec![-1]);
    assert_eq!(find_ball(vec![vec![1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, -1], vec![1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, -1]]), vec![0, 1, 2, 3, 4, -1]);
}
