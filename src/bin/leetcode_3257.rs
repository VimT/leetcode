//! 放三个车的价值之和最大 II

/// 前后缀分解，枚举中间值
pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
    // 计算当前状态 + 这一行的最大值，次大值，第三大值
    fn update(p: &mut [(i64, usize)], rows: &Vec<i32>) {
        for (j, &x) in rows.iter().enumerate() {
            if x as i64 > p[0].0 {
                if p[0].1 != j {
                    if p[1].1 != j {
                        p[2] = p[1];
                    }
                    p[1] = p[0];
                }
                p[0] = (x as i64, j);
            } else if j != p[0].1 && x as i64 > p[1].0 {
                if p[1].1 != j {
                    p[2] = p[1];
                }
                p[1] = (x as i64, j);
            } else if j != p[0].1 && j != p[1].1 && x as i64 > p[2].0 {
                p[2] = (x as i64, j);
            }
        }
    }

    let m = board.len();
    let mut p = [(i64::MIN, usize::MAX); 3]; // 最大，次大，第三大
    let mut suf = vec![[(0, 0); 3]; m];
    for i in (0..m).rev() {
        update(&mut p, &board[i]);
        suf[i] = p.clone();
    }
    let mut result = i64::MIN;
    p = [(i64::MIN, usize::MAX); 3];
    for (i, row) in board[..m - 2].iter().enumerate() {
        update(&mut p, row);
        for (j2, &y) in board[i + 1].iter().enumerate() {  // 第二个
            for (x, j1) in p { // 第一个
                for (z, j3) in suf[i + 2] { // 第三个
                    if j1 != j2 && j2 != j3 && j3 != j1 {  // 都不同列
                        result = result.max(x + y as i64 + z);
                        break;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(board: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![vec![-3, 1, 1, 1], vec![-3, 1, -3, 1], vec![-3, 2, 1, 1]]), 4);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 15);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]), 3);
    }
    test(maximum_value_sum);
}
