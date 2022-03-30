//! 逃脱阻碍者

/// 看谁先到终点
pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let dis = target[0].abs() + target[1].abs();
    for ghost in ghosts {
        let ghost_dis = (ghost[0] - target[0]).abs() + (ghost[1] - target[1]).abs();
        if ghost_dis <= dis {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1]), true);
    assert_eq!(escape_ghosts(vec![vec![1, 0]], vec![2, 0]), false);
    assert_eq!(escape_ghosts(vec![vec![2, 0]], vec![1, 0]), false);
}
