//! 解数独

/// 优化：使用bitset，建立 rows, cols, 使用 | 操作，快速找出未使用的数字
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    fn inner(board: &mut Vec<Vec<char>>, cx: usize) -> bool {
        let mut x = 10;
        let mut y = 10;
        'out: for i in cx..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    x = i;
                    y = j;
                    break 'out;
                }
            }
        }
        if x == 10 && y == 10 {
            return true;
        }
        let mut has = std::collections::HashSet::new();
        let bx = x / 3 * 3;
        let by = y / 3 * 3;
        for i in 0..9 {
            has.insert(board[bx + i / 3][by + i % 3]);
            has.insert(board[x][i]);
            has.insert(board[i][y]);
        }
        for i in 0..9 {
            let c = (i + b'1') as char;
            if has.contains(&c) { continue; }
            board[x][y] = c;
            if inner(board, x) { return true; }
            board[x][y] = '.';
        }

        false
    }
    inner(board, 0);
}

fn main() {
    let mut board = vec![
        vec![5, 3, -1, -1, 7, -1, -1, -1, -1].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![6, -1, -1, 1, 9, 5, -1, -1, -1].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![-1, 9, 8, -1, -1, -1, -1, 6, -1].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![8, -1, -1, -1, 6, -1, -1, -1, 3].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![4, -1, -1, 8, -1, 3, -1, -1, 1].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![7, -1, -1, -1, 2, -1, -1, -1, 6].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![-1, 6, -1, -1, -1, -1, 2, 8, -1].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![-1, -1, -1, 4, 1, 9, -1, -1, 5].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
        vec![-1, -1, -1, -1, 8, -1, -1, 7, 9].into_iter().map(|x| if x > 0 { (x as u8 + b'0') as char } else { '.' }).collect(),
    ];
    solve_sudoku(&mut board);
    for i in board {
        println!("{:?}", i);
    }
}