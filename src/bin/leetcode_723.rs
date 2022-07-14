//! 粉碎糖果

/// 官方解法，很巧妙
/// 1. 尾递归
/// 2. 使用负数标记是否消除，不用新建矩阵
pub fn candy_crush(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = board.len();
    let n = board[0].len();
    let mut todo = false;
    for i in 0..m {
        for j in 0..n - 2 {
            let v = board[i][j].abs();
            if v != 0 && v == board[i][j + 1].abs() && v == board[i][j + 2].abs() {
                board[i][j] = -v;
                board[i][j + 1] = -v;
                board[i][j + 2] = -v;
                todo = true;
            }
        }
    }
    for i in 0..m - 2 {
        for j in 0..n {
            let v = board[i][j].abs();
            if v != 0 && v == board[i + 1][j].abs() && v == board[i + 2][j].abs() {
                board[i][j] = -v;
                board[i + 1][j] = -v;
                board[i + 2][j] = -v;
                todo = true;
            }
        }
    }
    for j in 0..n {
        let mut wr = m;
        for i in (0..m).rev() {
            if board[i][j] > 0 {
                board[wr - 1][j] = board[i][j];
                wr -= 1;
            }
        }
        while wr > 0 {
            board[wr - 1][j] = 0;
            wr -= 1;
        }
    }
    if todo { candy_crush(board) } else { board }
}

fn main() {
    fn test(func: fn(board: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![110, 5, 112, 113, 114], vec![210, 211, 5, 213, 214], vec![310, 311, 3, 313, 314], vec![410, 411, 412, 5, 414], vec![5, 1, 512, 3, 3], vec![610, 4, 1, 613, 614], vec![710, 1, 2, 713, 714], vec![810, 1, 2, 1, 1], vec![1, 1, 2, 2, 2], vec![4, 1, 4, 4, 1014]]), vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![110, 0, 0, 0, 114], vec![210, 0, 0, 0, 214], vec![310, 0, 0, 113, 314], vec![410, 0, 0, 213, 414], vec![610, 211, 112, 313, 614], vec![710, 311, 412, 613, 714], vec![810, 411, 512, 713, 1014]]);
        assert_eq!(func(vec![vec![1, 3, 5, 5, 2], vec![3, 4, 3, 3, 1], vec![3, 2, 4, 5, 2], vec![2, 4, 4, 5, 5], vec![1, 4, 4, 1, 1]]), vec![vec![1, 3, 0, 0, 0], vec![3, 4, 0, 5, 2], vec![3, 2, 0, 3, 1], vec![2, 4, 0, 5, 2], vec![1, 4, 3, 1, 1]]);
    }
    test(candy_crush);
}
