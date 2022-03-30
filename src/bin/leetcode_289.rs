//! 生命游戏

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let m = board.len() as isize;
    let n = board[0].len() as isize;
    for i in 0..m as usize {
        for j in 0..n as usize {
            let mut live = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 { continue; }
                    let a = i as isize + x;
                    let b = j as isize + y;
                    if a < m && a >= 0 && b < n && b >= 0 && board[a as usize][b as usize].abs() == 1 {
                        // println!("({}, {}) has live ({}, {})", i, j, a, b);
                        live += 1;
                    }
                }
            }
            if board[i][j] == 1 && (live < 2 || live > 3) {
                board[i][j] = -1;
            }
            if board[i][j] == 0 && live == 3 {
                board[i][j] = 2;
            }
            // println!("({},{}) has {} live, current: {:?}", i, j, live, board);
        }
    }
    for i in 0..m as usize {
        for j in 0..n as usize {
            if board[i][j] > 0 {
                board[i][j] = 1;
            } else {
                board[i][j] = 0;
            }
        }
    }
}


fn main() {
    fn test(func: fn(board: &mut Vec<Vec<i32>>)) {
        let help = |mut board: Vec<Vec<i32>>| {
            func(&mut board);
            board
        };
        assert_eq!(help(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]]), vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]);
        assert_eq!(help(vec![vec![1, 1], vec![1, 0]]), vec![vec![1, 1], vec![1, 1]]);
    }
    test(game_of_life);
}
