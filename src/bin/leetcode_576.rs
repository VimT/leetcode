//! 出界的路径数

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    const MOD: i32 = 1e9 as i32 + 7;
    fn dfs(m: i32, n: i32, left: i32, x: i32, y: i32, cache: &mut Vec<Vec<Vec<Option<i32>>>>) -> i32 {
        if left < 0 { return 0; }
        if x < 0 || x >= m || y < 0 || y >= n {
            return 1;
        }
        if let Some(v) = cache[x as usize][y as usize][left as usize] {
            return v;
        }
        let mut result = 0;
        for (dx, dy) in DIR {
            let (nx, ny) = (x + dx, y + dy);
            result = (result + dfs(m, n, left - 1, nx, ny, cache)) % MOD;
        }
        cache[x as usize][y as usize][left as usize] = Some(result);
        result
    }
    dfs(m, n, max_move, start_row, start_column, &mut vec![vec![vec![None; max_move as usize + 1]; n as usize]; m as usize])
}

fn main() {
    assert_eq!(find_paths(8, 50, 23, 5, 26), 914783380);
    assert_eq!(find_paths(2, 2, 2, 0, 0), 6);
    assert_eq!(find_paths(1, 3, 3, 0, 1), 12);
}
