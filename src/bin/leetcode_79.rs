//! 单词搜索

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn inner(board: &Vec<Vec<char>>, mut word_index: usize, word: &Vec<char>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> bool {
        if word_index == word.len() {
            return true;
        }
        if visited[x][y] {
            return false;
        }
        if x >= board.len() || y >= board[0].len() {
            return false;
        }
        if word[word_index] != board[x][y] {
            return false;
        }
        // println!("current access: board[{}][{}]: {}, {}", x, y, board[x][y], word_index);

        visited[x][y] = true;
        word_index += 1;
        if (x > 0 && inner(board, word_index, word, x - 1, y, visited))
            || (y > 0 && inner(board, word_index, word, x, y - 1, visited))
            || (x < board.len() && inner(board, word_index, word, x + 1, y, visited))
            || (y < board[0].len() && inner(board, word_index, word, x, y + 1, visited)) {
            return true;
        }
        visited[x][y] = false;

        false
    }
    let visited = vec![vec![false; board[0].len() + 1]; board.len() + 1];
    let word_vec = word.chars().collect::<Vec<char>>();

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if inner(&board, 0, &word_vec, i, j, &mut visited.clone()) {
                return true;
            }
        }
    }
    false
}

pub fn exist_other(board: Vec<Vec<char>>, word: String) -> bool {
    fn dfs(board: &Vec<Vec<char>>, word: &[char], x: usize, y: usize, mem: &mut Vec<Vec<bool>>) -> bool {
        if word.len() == 0 {
            return true;
        }
        if mem[x][y] == true {
            return false;
        }
        let m = board.len();
        let n = board[0].len();
        let mut res = false;

        if board[x][y] == word[0] {
            if word.len() == 1 {
                return true;
            }
            //已经访问过，避免再访问。
            mem[x][y] = true;
            //上下左右搜索
            //优化2：使用&[char] 可以用引用
            if x + 1 < m { res = dfs(board, &word[1..], x + 1, y, mem); }
            if x > 0 { res = res || dfs(board, &word[1..], x - 1, y, mem) }
            if y + 1 < n { res = res || dfs(board, &word[1..], x, y + 1, mem); }
            if y > 0 { res = res || dfs(board, &word[1..], x, y - 1, mem); }
            if res { return true; }
            mem[x][y] = false;
        }
        return false;
    }
    let word: Vec<char> = word.chars().collect();
    if board.len() == 0 {
        return false;
    }
    let m = board.len();
    let n = board[0].len();
    let row: Vec<bool> = std::iter::repeat(false).take(n).collect();
    let mut mem = std::iter::repeat(row).take(m).collect();

    for i in 0..m {
        for j in 0..n {
            if word[0] == board[i][j] {
                // 优化1：提前判断依次
                if dfs(&board, &word[..], i, j, &mut mem) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn main() {
    fn test(func: fn(board: Vec<Vec<char>>, word: String) -> bool) {
        assert_eq!(func(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], String::from("ABCCED")), true);
        assert_eq!(func(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], String::from("SEE")), true);
        assert_eq!(func(vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']], String::from("ABCB")), false);
    }
    test(exist);
    test(exist_other);
}
