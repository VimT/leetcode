//! 岛屿的周长

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = 0;
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                let mut bian = 4;
                for &(dx, dy) in &dir {
                    let (x, y) = (i as i32 + dx, j as i32 + dy);
                    if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        if grid[x as usize][y as usize] == 1 {
                            bian -= 1;
                        }
                    }
                }
                result += bian;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(island_perimeter(vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]]), 16);
    assert_eq!(island_perimeter(vec![vec![1]]), 4);
    assert_eq!(island_perimeter(vec![vec![1, 0]]), 4);
}
