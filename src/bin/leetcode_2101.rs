//! 引爆最多的炸弹

use std::collections::VecDeque;

pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let len = bombs.len();
    let mut dp = vec![vec![false; len]; len];
    for i in 0..len {
        for j in 0..len {
            let dx = (bombs[i][0] - bombs[j][0]) as f64;
            let dy = (bombs[i][1] - bombs[j][1]) as f64;
            let dis = (dx * dx + dy * dy).sqrt();
            dp[i][j] = dis <= bombs[i][2] as f64;
        }
    }
    let mut q = VecDeque::new();
    let mut result = 0;
    for i in 0..len {
        let mut vis = vec![false; len];
        q.push_back(i);
        vis[i] = true;
        let mut num = 1;
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            for j in 0..len {
                if dp[cur][j] && !vis[j] {
                    q.push_back(j);
                    num += 1;
                    vis[j] = true;
                }
            }
        }
        result = result.max(num);
    }
    result
}

fn main() {
    assert_eq!(maximum_detonation(vec![vec![656, 619, 56], vec![189, 402, 178], vec![513, 373, 276], vec![900, 510, 14], vec![188, 173, 129], vec![512, 178, 251], vec![145, 685, 47], vec![504, 355, 500], vec![554, 131, 214], vec![596, 1, 98], vec![358, 230, 197], vec![88, 758, 155], vec![72, 340, 419], vec![818, 708, 222]]), 14);
    assert_eq!(maximum_detonation(vec![vec![54, 95, 4], vec![99, 46, 3], vec![29, 21, 3], vec![96, 72, 8], vec![49, 43, 3], vec![11, 20, 3], vec![2, 57, 1], vec![69, 51, 7], vec![97, 1, 10], vec![85, 45, 2], vec![38, 47, 1], vec![83, 75, 3], vec![65, 59, 3], vec![33, 4, 1], vec![32, 10, 2], vec![20, 97, 8], vec![35, 37, 3]]), 1);
    assert_eq!(maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]), 2);
    assert_eq!(maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]]), 1);
    assert_eq!(maximum_detonation(vec![vec![1, 2, 3], vec![2, 3, 1], vec![3, 4, 2], vec![4, 5, 3], vec![5, 6, 4]]), 5);
}
