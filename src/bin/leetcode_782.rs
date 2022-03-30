//! 变为棋盘

use std::collections::HashMap;

fn analyze_count(cnt: &HashMap<u32, u32>, len: u32) -> i32 {
    if cnt.len() != 2 { return -1; }
    let kv: Vec<(u32, u32)> = cnt.iter().map(|(&x, &y)| (x, y)).collect();
    let (k1, v1) = kv[0];
    let (k2, v2) = kv[1];
    if !(v1 == len / 2 && v2 == (len + 1) / 2) && !(v2 == len / 2 && v1 == (len + 1) / 2) {
        return -1;
    }
    if k1 ^ k2 != (1 << len) - 1 {
        return -1;
    }
    let nones = (1 << len) - 1;
    let ones = (k1 & nones).count_ones();
    let mut cand = u32::MAX;
    if len % 2 == 0 || ones as u32 * 2 < len {
        // zero start
        // 计算k1 变成 101010 需要多少位发生变化，使用异或计算
        cand = cand.min(((k1 ^ 0xAAAAAAAA & nones).count_ones() / 2) as u32);
    }
    if len % 2 == 0 || ones as u32 * 2 > len {
        cand = cand.min(((k1 ^ 0x55555555 & nones).count_ones() / 2) as u32);
    }

    cand as i32
}

pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    let len = board.len();
    let mut cnt = HashMap::new();
    for row in &board {
        let mut code = 0;
        for &x in row {
            code = code * 2 + x as u32;
        }
        *cnt.entry(code).or_insert(0u32) += 1;
    }

    let k1 = analyze_count(&cnt, len as u32);
    if k1 == -1 { return -1; }
    cnt.clear();
    for j in 0..len {
        let mut code = 0;
        for i in 0..len {
            code = code * 2 + board[i][j] as u32;
        }
        *cnt.entry(code).or_default() += 1;
    }
    let k2 = analyze_count(&cnt, len as u32);
    if k2 >= 0 { k1 + k2 } else { -1 }
}

fn main() {
    assert_eq!(moves_to_chessboard(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 1]]), 2);
    assert_eq!(moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]), 0);
    assert_eq!(moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]), -1);
}
