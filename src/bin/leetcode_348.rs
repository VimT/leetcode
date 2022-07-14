//! 设计井字棋

struct TicTacToe {
    n: i32,
    rows: [Vec<i32>; 2],
    cols: [Vec<i32>; 2],
    diag: [[i32; 2]; 2],
}


impl TicTacToe {
    fn new(n: i32) -> Self {
        Self {
            n,
            rows: [vec![0; n as usize], vec![0; n as usize]],
            cols: [vec![0; n as usize], vec![0; n as usize]],
            diag: [[0, 0], [0, 0]],
        }
    }

    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let pi = player as usize - 1;
        let row = row as usize;
        let col = col as usize;
        self.rows[pi][row] += 1;
        if self.rows[pi][row] == self.n { return player; }
        self.cols[pi][col] += 1;
        if self.cols[pi][col] == self.n { return player; }
        if row == col {
            self.diag[pi][0] += 1;
            if self.diag[pi][0] == self.n { return player; }
        }
        if row + col == self.n as usize - 1 {
            self.diag[pi][1] += 1;
            if self.diag[pi][1] == self.n { return player; }
        }
        0
    }
}


fn main() {
    let mut ttt = TicTacToe::new(3);
    assert_eq!(ttt.make_a_move(0, 0, 1), 0);
    assert_eq!(ttt.make_a_move(0, 2, 2), 0);
    assert_eq!(ttt.make_a_move(2, 2, 1), 0);
    assert_eq!(ttt.make_a_move(1, 1, 2), 0);
    assert_eq!(ttt.make_a_move(2, 0, 1), 0);
    assert_eq!(ttt.make_a_move(1, 0, 2), 0);
    assert_eq!(ttt.make_a_move(2, 1, 1), 1);
}
