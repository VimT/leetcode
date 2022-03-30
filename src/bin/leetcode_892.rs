//! 三维形体的表面积

pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let dir = [-1, 0, 1, 0, -1];
    let mut result = 0;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] > 0 {
                let mut num = 6 * grid[i][j];
                num -= 2 * (grid[i][j] - 1);
                for d in 0..4 {
                    let (nx, ny) = (i as i32 + dir[d], j as i32 + dir[d + 1]);
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        num -= grid[i][j].min(grid[nx][ny]);
                    }
                }
                result += num;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(surface_area(vec![vec![2]]), 10);
    assert_eq!(surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(surface_area(vec![vec![1, 0], vec![0, 2]]), 16);
    assert_eq!(surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]), 32);
    assert_eq!(surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]), 46);
}
