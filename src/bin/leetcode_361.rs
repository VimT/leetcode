//! 轰炸敌人

/// 暴力，可以优化，提前计算好，每个格子各方向的E数量，例如向左： left[i] = left[i-1] + 1 if i is E
pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut boom = vec![vec![0; n]; m];
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'E' {
                for (dx, dy) in DIR {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    loop {
                        x += dx;
                        y += dy;
                        if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                            break;
                        }
                        if grid[x as usize][y as usize] == 'W' {
                            break;
                        }
                        boom[x as usize][y as usize] += 1;
                    }
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '0' {
                result = result.max(boom[i][j]);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<char>>) -> i32) {
        assert_eq!(func(vec![vec!['0', 'E', '0', '0'], vec!['E', '0', 'W', 'E'], vec!['0', 'E', '0', '0']]), 3);
        assert_eq!(func(vec![vec!['W', 'W', 'W'], vec!['0', '0', '0'], vec!['E', 'E', 'E']]), 1);
    }
    test(max_killed_enemies);
}
