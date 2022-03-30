//! 翻转卡片游戏

pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    let mut cannt = vec![None; 2001];
    for (&front, &back) in fronts.iter().zip(&backs) {
        if front == back {
            cannt[front as usize] = Some(true);
        } else {
            if cannt[front as usize].is_none() {
                cannt[front as usize] = Some(false);
            }
            if cannt[back as usize].is_none() {
                cannt[back as usize] = Some(false);
            }
        }
    }
    for i in 0..=2000 {
        if let Some(v) = cannt[i] {
            if !v {
                return i as i32;
            }
        }
    }
    0
}

fn main() {
    assert_eq!(flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]), 2);
    assert_eq!(flipgame(vec![1], vec![1]), 0);
}
