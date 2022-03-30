//! 砖墙

use std::collections::HashMap;

pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    let mut m = HashMap::new();
    let len = wall.len();
    for line in wall {
        let mut cur = 0;
        for &zhuan in &line[..line.len() - 1] {
            cur += zhuan;
            *m.entry(cur).or_default() += 1;
        }
    }
    len as i32 - *m.values().max().unwrap_or(&0)
}

fn main() {
    assert_eq!(least_bricks(vec![vec![1, 2, 2, 1], vec![3, 1, 2], vec![1, 3, 2], vec![2, 4], vec![3, 1, 2], vec![1, 3, 1, 1]]), 2);
    assert_eq!(least_bricks(vec![vec![1], vec![1], vec![1]]), 3);
}
