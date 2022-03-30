//! 有效的井字游戏

use leetcode::svec;

pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let mut o: i8 = 0;
    let mut x = 0;
    let board: Vec<&[u8]> = board.iter().map(|x| x.as_bytes()).collect();
    for i in 0..3 {
        for j in 0..3 {
            match board[i][j] {
                b'O' => o += 1,
                b'X' => x += 1,
                b' ' => {}
                _ => panic!()
            }
        }
    }
    if !(x == o || o + 1 == x) {
        return false;
    }

    for i in 0..3 {
        if board[i][0] != b' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            if board[i][0] == b'O' { if o != x { return false; } } else { if x - 1 != o { return false; } };
        }
        if board[0][i] != b' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            if board[0][i] == b'O' { if o != x { return false; } } else { if x - 1 != o { return false; } };
        }
    }
    if board[0][0] != b' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if board[0][0] == b'O' { if o != x { return false; } } else { if x - 1 != o { return false; } };
    }
    if board[0][2] != b' ' && board[0][2] == board[1][1] && board[2][0] == board[1][1] {
        if board[0][2] == b'O' { if o != x { return false; } } else { if x - 1 != o { return false; } };
    }
    true
}

fn main() {
    assert_eq!(valid_tic_tac_toe(svec!["XOX", "OOX", "XO "]), true);
    assert_eq!(valid_tic_tac_toe(svec!["O  ", "   ", "   "]), false);
    assert_eq!(valid_tic_tac_toe(svec!["XOX", " X ", "   "]), false);
    assert_eq!(valid_tic_tac_toe(svec!["XXX", "   ", "OOO"]), false);
    assert_eq!(valid_tic_tac_toe(svec!["XOX", "O O", "XOX"]), true);
}
