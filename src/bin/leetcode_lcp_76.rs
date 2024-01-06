//! 魔法棋盘


/// 可能有7种状态
/// 0. 空
/// 1. B
/// 2. R
/// 3. 全B
/// 4. 全R
/// 5. BR交替，以B结尾
/// 6. BR交替，以R结尾
///
/// 1 <= m*n <= 30，意味着旋转后，保证列数 <=5
/// 用3比特表示7种状态， 每一行最多用 15个比特表示
pub fn get_scheme_count(n: i32, m: i32, chessboard: Vec<String>) -> i64 {
    // 当前状态在 填B / 填R 后的状态转移
    static TRANS: [[i32; 2]; 7] = [
        [1, 2], //0
        [3, 6], //1
        [5, 4], //2
        [3, -1], //3
        [-1, 4], //4
        [-1, 6], //5
        [5, -1], //6
    ];

    let mut n = n as usize;
    let mut m = m as usize;
    let mut board: Vec<Vec<u8>> = chessboard.into_iter().map(|x| x.into_bytes()).collect();
    if n < m {
        let mut new_board = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                new_board[j][i] = board[i][j];
            }
        }
        board = new_board;
        std::mem::swap(&mut n, &mut m);
    }

    struct DFS {
        board: Vec<Vec<u8>>,
        cache: Vec<Vec<i64>>,
        n: usize,
        m: usize,
    }

    impl DFS {
        /// 轮廓线dp：所有的列是轮流处理的，行则是每次处理一整行，因此需要记录所有列和当前行的状态。
        /// 枚举到第i行，当前每一列的状态为 col_mask
        fn dfs(&mut self, i: usize, col_mask: i32) -> i64 {
            if i == self.n { return 1; }

            if self.cache[i][col_mask as usize] != -1 {
                return self.cache[i][col_mask as usize];
            }

            let result = self.dfs2(i, 0, 0, col_mask);
            self.cache[i][col_mask as usize] = result;
            result
        }

        // (i,j) 填颜色 color，判断是否合法，如果合法生成新的 row_mask 和 col_mask
        fn next(&mut self, i: usize, j: usize, row_mask: i32, col_mask: i32, color: usize) -> i64 {
            let row_next = TRANS[row_mask as usize][color];
            if row_next < 0 { return 0; }
            let j3 = j * 3;
            let mut col_next = TRANS[(col_mask as usize >> j3) & 7][color];
            if col_next < 0 { return 0; }
            col_next = col_mask & !(7 << j3) | (col_next << j3);
            return self.dfs2(i, j + 1, row_next, col_next);
        }

        // 爆搜所有合法的构造方案
        fn dfs2(&mut self, i: usize, j: usize, row_mask: i32, col_mask: i32) -> i64 {
            if j == self.m { return self.dfs(i + 1, col_mask); }
            let b = self.board[i][j];
            match b {
                b'B' => self.next(i, j, row_mask, col_mask, 0),
                b'R' => self.next(i, j, row_mask, col_mask, 1),
                b'.' => self.dfs2(i, j + 1, row_mask, col_mask),
                _ => self.dfs2(i, j + 1, row_mask, col_mask) + self.next(i, j, row_mask, col_mask, 0) + self.next(i, j, row_mask, col_mask, 1)
            }
        }
    }
    let mut dfs = DFS { board, cache: vec![vec![-1; 1 << (3 * m)]; n], n, m };
    dfs.dfs(0, 0)
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(n: i32, m: i32, chessboard: Vec<String>) -> i64) {
        assert_eq!(func(3, 3, svec!["..R","..B","?R?"]), 5);
        assert_eq!(func(3, 3, svec!["?R?","B?B","?R?"]), 105);
    }
    test(get_scheme_count);
}
