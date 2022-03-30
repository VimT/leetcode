//! 被围绕的区域

use std::collections::VecDeque;

pub fn solve(board: &mut Vec<Vec<char>>) {
    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if board[i][j] == 'X' || board[i][j] == '#' {
            return;
        }
        board[i][j] = '#';
        if i > 0 { dfs(board, i - 1, j); }
        if i < board.len() - 1 { dfs(board, i + 1, j); }
        if j > 0 { dfs(board, i, j - 1); }
        if j < board[0].len() - 1 { dfs(board, i, j + 1); }
    }
    let m = board.len();
    if m == 0 { return; }
    let n = board[0].len();
    for i in 0..m {
        for j in 0..n {
            let is_edge = i == 0 || j == 0 || i == m - 1 || j == n - 1;
            if is_edge && board[i][j] == 'O' {
                dfs(board, i, j);
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'O' {
                board[i][j] = 'X';
            }
            if board[i][j] == '#' {
                board[i][j] = 'O';
            }
        }
    }
}

pub fn solve_bfs(board: &mut Vec<Vec<char>>) {
    fn bfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let mut q = VecDeque::new();
        q.push_back((i, j));
        board[i][j] = '#';
        while !q.is_empty() {
            let (a, b) = q.pop_front().unwrap();
            if a > 0 && board[a - 1][b] == 'O' {
                q.push_back((a - 1, b));
                board[a - 1][b] = '#';
            }
            if a + 1 < board.len() && board[a + 1][b] == 'O' {
                q.push_back((a + 1, b));
                board[a + 1][b] = '#';
            }
            if b > 0 && board[a][b - 1] == 'O' {
                q.push_back((a, b - 1));
                board[a][b - 1] = '#';
            }
            if b + 1 < board[0].len() && board[a][b + 1] == 'O' {
                q.push_back((a, b + 1));
                board[a][b + 1] = '#';
            }
        }
    }
    let m = board.len();
    if m == 0 { return; }
    let n = board[0].len();
    for i in 0..m {
        for j in 0..n {
            let is_edge = i == 0 || j == 0 || i == m - 1 || j == n - 1;
            if is_edge && board[i][j] == 'O' {
                bfs(board, i, j);
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'O' { board[i][j] = 'X'; }
            if board[i][j] == '#' { board[i][j] = 'O'; }
        }
    }
}

fn main() {
    fn test(func: fn(board: &mut Vec<Vec<char>>)) {
        let help = |mut board: Vec<Vec<char>>| {
            func(&mut board);
            board
        };
        assert_eq!(help(vec![vec!['X', 'X', 'X', 'X'], vec!['X', 'O', 'O', 'X'], vec!['X', 'X', 'O', 'X'], vec!['X', 'O', 'X', 'X']]), vec![vec!['X', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'X'], vec!['X', 'O', 'X', 'X']]);
        assert_eq!(help(vec![vec!['X']]), vec![vec!['X']]);
    }
    test(solve);
    test(solve_bfs);
}
