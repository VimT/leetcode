//! N皇后 II

pub fn total_n_queens(n: i32) -> i32 {
    fn check(row: usize, col: usize, n: usize, rows: &Vec<u8>, hills: &Vec<u8>, dales: &Vec<u8>) -> bool {
        rows[col] + hills[row + n - col] + dales[row + col] == 0
    }
    /// hills: 左上 -> 右下 情况
    /// dales: 右上 -> 左下 情况
    /// rows: 列情况
    fn backtrace(row: usize, mut count: usize, n: usize, rows: &mut Vec<u8>, hills: &mut Vec<u8>, dales: &mut Vec<u8>) -> usize {
        for col in 0..n {
            if check(row, col, n, rows, hills, dales) {
                rows[col] = 1;
                hills[row + n - col] = 1;
                dales[row + col] = 1;

                count = if row + 1 == n { count + 1 } else { backtrace(row + 1, count, n, rows, hills, dales) };

                rows[col] = 0;
                hills[row + n - col] = 0;
                dales[row + col] = 0;
            }
        }
        count
    }
    let n = n as usize;
    let mut rows = vec![0; n];
    let mut hills = vec![0; 2 * n];
    let mut dales = vec![0; 2 * n - 1];
    backtrace(0, 0, n, &mut rows, &mut hills, &mut dales) as i32
}

/// 位运算法
/// x & -x  除最后一个1保留，其他全为0
/// x & (x - 1) 最后一个1变成0
pub fn total_n_queens_bits(n: i32) -> i32 {
    /// next_row  下一行被占据情况
    /// 0b1111 表示n=4 时，所有行都占满
    fn backtrace(row: i32, hills: i32, next_row: i32, dales: i32, mut count: i32, n: i32) -> i32 {
        let cols = (1 << n) - 1;
        if row == n {
            count += 1;
        } else {
            // 当前行可用的列
            let mut free_cols = cols & !(hills | next_row | dales);

            while free_cols != 0 {
                // free_cols 第一个 为 1 的位
                let current_col = -free_cols & free_cols;
                // 放置皇后，并排除对应的列
                free_cols ^= current_col;
                count = backtrace(row + 1, (hills | current_col) << 1, next_row | current_col, (dales | current_col) >> 1, count, n);
            }
        }

        count
    }
    backtrace(0, 0, 0, 0, 0, n)
}


fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(4), 2);
        assert_eq!(func(1), 1);
    }
    test(total_n_queens);
    test(total_n_queens_bits);
}
