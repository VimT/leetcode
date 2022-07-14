//! 统计网格图中没有被保卫的格子数

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Status {
    Unknown,
    Wall,
    Guard,
    Safe,
}

impl Status {
    fn can_pass(self) -> bool {
        match self {
            Unknown => true,
            Safe => true,
            _ => false
        }
    }
}

use Status::*;

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut grid = vec![vec![Unknown; n]; m];
    for wall in walls {
        grid[wall[0] as usize][wall[1] as usize] = Wall;
    }
    for guard in &guards {
        grid[guard[0] as usize][guard[1] as usize] = Guard;
    }
    for guard in &guards {
        let x = guard[0] as usize;
        let y = guard[1] as usize;
        let mut dx = x;
        while dx > 0 && grid[dx - 1][y].can_pass() {
            grid[dx - 1][y] = Safe;
            dx -= 1;
        }
        dx = x + 1;
        while dx < m && grid[dx][y].can_pass() {
            grid[dx][y] = Safe;
            dx += 1;
        }
        let mut dy = y;
        while dy > 0 && grid[x][dy - 1].can_pass() {
            grid[x][dy - 1] = Safe;
            dy -= 1;
        }
        dy = y + 1;
        while dy < n && grid[x][dy].can_pass() {
            grid[x][dy] = Safe;
            dy += 1;
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == Unknown {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(count_unguarded(4, 6, vec![vec![0, 0], vec![1, 1], vec![2, 3]], vec![vec![0, 1], vec![2, 2], vec![1, 4]]), 7);
    assert_eq!(count_unguarded(3, 3, vec![vec![1, 1]], vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]]), 4);
}
