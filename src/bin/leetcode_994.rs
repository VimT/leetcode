//! 腐烂的橘子

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let dir = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut qs = vec![];
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 2 {
                result = -1;
                grid[i][j] = 1;
                qs.push(vec![(i as i32, j as i32)]);
            }
        }
    }

    while !qs.is_empty() {
        let mut new_qs = vec![];
        let mut has = false;
        for q in qs {
            let mut new_q = vec![];
            for pos in q {
                if grid[pos.0 as usize][pos.1 as usize] == 2 {
                    continue;
                }
                has = true;
                grid[pos.0 as usize][pos.1 as usize] = 2;
                for &dir in &dir {
                    let x = pos.0 + dir.0;
                    let y = pos.1 + dir.1;
                    if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        if grid[x as usize][y as usize] == 1 {
                            new_q.push((x, y));
                        }
                    }
                }
            }
            if !new_q.is_empty() {
                new_qs.push(new_q);
            }
        }
        qs = new_qs;
        if has {
            result += 1;
        }
    }
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                return -1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]), 4);
    assert_eq!(oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]), -1);
    assert_eq!(oranges_rotting(vec![vec![0, 2]]), 0);
}
