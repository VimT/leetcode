//! 隔离病毒

use std::collections::{HashSet, VecDeque};

pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
    let m = is_infected.len();
    let n = is_infected[0].len();
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let neighbors = |x: usize, y: usize| {
        DIR.iter().filter_map(move |&(dx, dy)| {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                return Some((nx as usize, ny as usize));
            }
            None
        })
    };
    let mut result = 0;
    let mut regions = vec![];
    let mut q = VecDeque::new();
    loop {
        let mut seen = vec![vec![false; n]; m];
        regions.clear();
        for i in 0..m {
            for j in 0..n {
                if is_infected[i][j] == 1 && !seen[i][j] {
                    q.push_back((i, j));
                    seen[i][j] = true;
                    let mut region = vec![];
                    let mut zhou = HashSet::new();
                    let mut perimeter = 0;
                    while !q.is_empty() {
                        let (x, y) = q.pop_front().unwrap();
                        region.push((x, y));
                        for (nx, ny) in neighbors(x, y) {
                            if is_infected[nx][ny] == 1 {
                                if !seen[nx][ny] {
                                    q.push_back((nx, ny));
                                    seen[nx][ny] = true;
                                }
                            } else if is_infected[nx][ny] == 0 {
                                zhou.insert((nx, ny));
                                // 这样8个1围1个0的情况，这个0会被算4个防火墙，符合现实
                                perimeter += 1;
                            }
                        }
                    }
                    regions.push((region, zhou, perimeter));
                }
            }
        }
        if regions.is_empty() { break; }
        let max_idx = (0..regions.len()).max_by_key(|&x| regions[x].1.len()).unwrap();
        result += regions[max_idx].2;
        for i in 0..regions.len() {
            if i == max_idx {
                for &(x, y) in &regions[i].0 {
                    is_infected[x][y] = 2;
                }
            } else {
                for &(x, y) in &regions[i].1 {
                    if is_infected[x][y] == 0 {
                        is_infected[x][y] = 1;
                    }
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(contain_virus(vec![vec![0, 1, 0, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 0, 0, 0, 1, 0], vec![0, 0, 0, 1, 1, 0, 0, 1, 1, 0], vec![0, 1, 0, 0, 1, 0, 1, 1, 0, 1], vec![0, 0, 0, 1, 0, 1, 0, 1, 1, 1], vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 0], vec![0, 1, 0, 1, 0, 0, 0, 1, 1, 0], vec![0, 1, 1, 0, 0, 1, 1, 0, 0, 1], vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1]]
    ), 38);
    assert_eq!(contain_virus(vec![vec![1, 1, 1, 0, 0, 0, 0, 0, 0], vec![1, 0, 1, 0, 1, 1, 1, 1, 1], vec![1, 1, 1, 0, 0, 0, 0, 0, 0]]), 13);
    assert_eq!(contain_virus(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]), 4);
    assert_eq!(contain_virus(vec![vec![0, 1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 10);
}
