//! 转化为全零矩阵的最少反转次数

use std::collections::VecDeque;

fn vec2bit(mat: &Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                result |= 1 << (i * n + j);
            }
        }
    }
    result
}

pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut convert = vec![];
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 0..m {
        for j in 0..n {
            let mut c = vec![vec![0; n]; m];
            c[i][j] = 1;
            for (dx, dy) in DIR {
                let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    c[nx as usize][ny as usize] = 1;
                }
            }
            convert.push(vec2bit(&c));
        }
    }
    let mut q = VecDeque::new();
    let stat = vec2bit(&mat);
    let mut seen = vec![false; 1 << m * n];
    seen[stat as usize] = true;
    q.push_back((stat, 0));
    while !q.is_empty() {
        let (stat, step) = q.pop_front().unwrap();
        if stat == 0 {
            return step;
        }
        for &conv in &convert {
            let next = stat ^ conv;
            if !seen[next as usize] {
                q.push_back((next, step + 1));
                seen[next as usize] = true;
            }
        }
    }
    -1
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0], vec![0, 1]]), 3);
        assert_eq!(func(vec![vec![0]]), 0);
        assert_eq!(func(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
    }
    test(min_flips);
}
