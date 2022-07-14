//! 安卓系统手势解锁

static MUST: [[i32; 10]; 10] = {
    let mut result = [[-1; 10]; 10];
    result[1][7] = 4;
    result[1][9] = 5;
    result[1][3] = 2;
    result[2][8] = 5;
    result[3][1] = 2;
    result[3][9] = 6;
    result[3][7] = 5;
    result[4][6] = 5;
    result[6][4] = 5;
    result[7][1] = 4;
    result[7][9] = 8;
    result[7][3] = 5;
    result[8][2] = 5;
    result[9][7] = 8;
    result[9][3] = 6;
    result[9][1] = 5;
    result
};

/// dp[state][i] 当前状态为state，最后一个连的点是i
pub fn number_of_patterns(m: i32, n: i32) -> i32 {
    fn dfs(m: i32, n: i32, state: i32, last: i32, result: &mut i32) {
        let has = state.count_ones() as i32;
        if has >= m && has <= n {
            *result += 1;
        }
        if has > n { return; }
        for i in 0..9 {
            if state >> i & 1 == 0 {
                // 检查last能不能到i
                let must_pass = MUST[last as usize + 1][i as usize + 1];
                if must_pass == -1 || state >> (must_pass - 1) & 1 == 1 {
                    dfs(m, n, state | 1 << i, i, result);
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..9 {
        dfs(m, n, 1 << i, i, &mut result);
    }
    result
}

fn main() {
    fn test(func: fn(m: i32, n: i32) -> i32) {
        assert_eq!(func(1, 1), 9);
        assert_eq!(func(1, 2), 65);
    }
    test(number_of_patterns);
}
