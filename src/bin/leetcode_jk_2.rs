//! 九坤-02. 池塘计数

use std::collections::VecDeque;
use leetcode::svec;

pub fn lake_count(mut field: Vec<String>) -> i32 {
    let m = field.len();
    let n = field[0].len();
    let mut q = VecDeque::new();
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if field[i].as_bytes()[j] == b'W' {
                unsafe { field[i].as_bytes_mut()[j] = b'.'; }
                q.push_back((i, j));
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for dx in [-1, 0, 1] {
                        for dy in [-1, 0, 1] {
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                                let (nx, ny) = (nx as usize, ny as usize);
                                if field[nx].as_bytes()[ny] == b'W' {
                                    unsafe { field[nx].as_bytes_mut()[ny] = b'.'; }
                                    q.push_back((nx, ny));
                                }
                            }
                        }
                    }
                }
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(lake_count(svec![".....W",".W..W.","....W.",".W....","W.WWW.",".W...."]), 3);
    assert_eq!(lake_count(svec!["W....W",".W..W.","..W.W.","W..W..","W.W...",".W...."]), 1);
}
