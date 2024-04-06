//! 出现频率最高的质数

use std::collections::HashMap;

fn is_prime(num: i32) -> bool {
    if num < 10 { return false; }
    for i in 2..=(num as f64).sqrt() as i32 {
        if num % i == 0 { return false; }
    }
    true
}

pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let (mut x, mut y) = (i as i32, j as i32);
                    let mut num = 0;
                    while x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        num = num * 10 + mat[x as usize][y as usize];
                        if is_prime(num) {
                            *cnt.entry(num).or_insert(0) += 1;
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
    }
    cnt.into_iter().max_by_key(|x| (x.1, x.0)).map(|x| x.0).unwrap_or(-1)
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1], vec![9, 9], vec![1, 1]]), 19);
        assert_eq!(func(vec![vec![7]]), -1);
        assert_eq!(func(vec![vec![9, 7, 8], vec![4, 6, 5], vec![2, 8, 6]]), 97);
    }
    test(most_frequent_prime);
}
