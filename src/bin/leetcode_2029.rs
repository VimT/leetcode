//! 石子游戏 IX

/// 0 的石子可以看成是「先后手」交换
/// 剩下的移除序列比较固定
/// 要么1121212121..
/// 要么2212121212..
pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    for stone in stones {
        match stone % 3 {
            2 => c2 += 1,
            1 => c1 += 1,
            0 => c0 += 1,
            _ => { unreachable!() }
        }
    }
    if c0 & 1 == 0 {
        return c1 > 0 && c2 > 0;
    }
    return c1 - c2 > 2 || c2 - c1 > 2;
}

fn main() {
    assert_eq!(stone_game_ix(vec![2, 1]), true);
    assert_eq!(stone_game_ix(vec![2]), false);
    assert_eq!(stone_game_ix(vec![5, 1, 2, 4, 3]), false);
}
