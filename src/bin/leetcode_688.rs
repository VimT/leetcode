//! “马”在棋盘上的概率

const POS: [(i32, i32); 8] = [(-2, -1), (-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2)];

pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    fn dfs(n: i32, k: i32, x: i32, y: i32, cache: &mut Vec<Vec<Vec<Option<f64>>>>) -> f64 {
        if !(x >= 0 && x < n && y >= 0 && y < n) {
            return 0.;
        }
        if let Some(v) = cache[x as usize][y as usize][k as usize] {
            return v;
        }
        if k == 0 { return 1.; }
        let mut result = 0.;
        for (dx, dy) in POS {
            let (nx, ny) = (x + dx, y + dy);
            result += dfs(n, k - 1, nx, ny, cache) / 8.;
        }
        cache[x as usize][y as usize][k as usize] = Some(result);
        result
    }
    dfs(n, k, row, column, &mut vec![vec![vec![None; 1 + k as usize]; n as usize]; n as usize])
}

fn main() {
    assert_eq!(knight_probability(3, 2, 0, 0), 0.06250);
    assert_eq!(knight_probability(1, 0, 0, 0), 1.00000);
}
