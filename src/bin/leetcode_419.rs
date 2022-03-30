//! 甲板上的战舰

pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let m = board.len();
    let n = board[0].len();
    let mut vis = vec![vec![false; n]; m];
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'X' && !vis[i][j] {
                vis[i][j] = true;
                let mut ni = i + 1;
                let mut nj = j + 1;
                if ni < m && board[ni][j] == 'X' && nj < n && board[i][nj] == 'X' {
                    result += 2;
                } else { result += 1; }
                while ni < m && board[ni][j] == 'X' {
                    vis[ni][j] = true;
                    ni += 1;
                }
                while nj < n && board[i][nj] == 'X' {
                    vis[i][nj] = true;
                    nj += 1;
                }
            }
        }
    }

    result
}

fn main() {
    assert_eq!(count_battleships(vec![vec!['X', '.', '.', 'X'], vec!['.', '.', '.', 'X'], vec!['.', '.', '.', 'X']]), 2);
    assert_eq!(count_battleships(vec![vec!['.']]), 0);
}
