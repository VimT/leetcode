//! 有效的数独


pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![vec![0; 9]; 9];
    let mut cols = vec![vec![0; 9]; 9];
    let mut boxes = vec![vec![0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != '.' {
                let num = (board[i][j] as u8 - b'1') as usize;
                let box_index = i / 3 * 3 + j / 3;
                rows[i][num] += 1;
                cols[j][num] += 1;
                boxes[box_index][num] += 1;

                if rows[i][num] > 1 || cols[j][num] > 1 || boxes[box_index][num] > 1 {
                    return false;
                }
            }
        }
    }
    true
}

pub fn is_valid_sudoku_bit(board: Vec<Vec<char>>) -> bool {
    for row_ind in 0..9 {
        let mut row_nums = 0u16;
        let mut col_nums = 0u16;
        let mut cell_nums = 0u16;
        for col_ind in 0..9 {
            let row_ele = board[row_ind][col_ind];
            if row_ele != '.' {
                let num = row_ele.to_digit(10).unwrap();
                if row_nums >> num & 1 == 1 {
                    return false;
                }
                row_nums = row_nums | 1 << num;
            }

            let col_ele = board[col_ind][row_ind];
            if col_ele != '.' {
                let num = col_ele.to_digit(10).unwrap();
                if col_nums >> num & 1 == 1 {
                    return false;
                }
                col_nums = col_nums | 1 << num;
            }

            let cell_ind = (row_ind / 3) * 27 + (row_ind % 3) * 3 + (col_ind % 3) + (col_ind / 3) * 9;
            let cell_row_ind = cell_ind / 9;
            let cell_col_ind = cell_ind % 9;
            let cell_ele = board[cell_row_ind][cell_col_ind];
            if cell_ele != '.' {
                let num = cell_ele.to_digit(10).unwrap();
                if cell_nums >> num & 1 == 1 {
                    return false;
                }
                cell_nums = cell_nums | 1 << num;
            }
        }
    }
    return true;
}

fn main() {
    fn test(func: fn(board: Vec<Vec<char>>) -> bool) {
        assert_eq!(func(vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']]), false);
        assert_eq!(func(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]), true);
    }
    test(is_valid_sudoku);
    test(is_valid_sudoku_bit);
}
