//! 参加考试的最大学生数

pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
    let m = seats.len();
    let n = seats[0].len();
    let seats: Vec<i32> = seats.into_iter().map(|x| {
        let mut num = 0;
        for i in 0..n {
            if x[i] == '#' { num |= 1 << i; }
        }
        num
    }).collect();
    let mut dp = vec![vec![0; 1 << n]; m]; // dp[i][j] 表示前i行，这一行状态是j的最大座位数
    for i in 0..1 << n {
        if i & seats[0] == 0 && i & i << 1 == 0 && i & i >> 1 == 0 {
            dp[0][i as usize] = i.count_ones();
        }
    }
    for i in 1..m {
        for j in 0..1 << n {
            if j & seats[i - 1] == 0 {  // 上一行的状态合法
                for k in 0..1 << n {
                    // 这一行的状态也合法, 用 k & k << 1 == 0 判断左右没有1
                    if k & seats[i] == 0 && k & k << 1 == 0 && k & k >> 1 == 0 && k >> 1 & j == 0 && j >> 1 & k == 0 {
                        dp[i][k as usize] = dp[i][k as usize].max(dp[i - 1][j as usize] + k.count_ones());
                    }
                }
            }
        }
    }
    dp[m - 1].iter().cloned().max().unwrap() as i32
}

/// 记忆化搜索
pub fn max_students2(seats: Vec<Vec<char>>) -> i32 {
    let m = seats.len();
    let n = seats[0].len();
    let seats: Vec<i32> = seats.into_iter().map(|x| {
        let mut num = 0;
        for i in 0..n {
            if x[i] == '#' { num |= 1 << i; }
        }
        num
    }).collect();
    fn dfs(seats: &Vec<i32>, n: usize, i: usize, last_state: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
        if i == seats.len() { return 0; }
        if cache[i][last_state as usize] != -1 {
            return cache[i][last_state as usize];
        }
        let mut result = 0;
        for state in 0..1 << n {
            if state & seats[i] == 0 && state & state << 1 == 0 && state & state >> 1 == 0 && state >> 1 & last_state == 0 && last_state >> 1 & state == 0 {
                result = result.max(state.count_ones() as i32 + dfs(seats, n, i + 1, state, cache));
            }
        }
        cache[i][last_state as usize] = result;
        result
    }
    dfs(&seats, n, 0, 0, &mut vec![vec![-1; 1 << n]; m])
}

fn main() {
    fn test(func: fn(seats: Vec<Vec<char>>) -> i32) {
        assert_eq!(func(vec![vec!['.', '.', '#', '#'],
                             vec!['.', '#', '.', '.'],
                             vec!['#', '.', '.', '#'],
                             vec!['#', '#', '#', '.']]), 4);
        assert_eq!(func(vec![vec!['#', '.', '.', '.', '#'],
                             vec!['.', '#', '.', '#', '.'],
                             vec!['.', '.', '#', '.', '.'],
                             vec!['.', '#', '.', '#', '.'],
                             vec!['#', '.', '.', '.', '#']]), 10);
        assert_eq!(func(vec![vec!['.', '#'],
                             vec!['#', '#'],
                             vec!['#', '.'],
                             vec!['#', '#'],
                             vec!['.', '#']]), 3);
        assert_eq!(func(vec![vec!['#', '.', '#', '#', '.', '#'],
                             vec!['.', '#', '#', '#', '#', '.'],
                             vec!['#', '.', '#', '#', '.', '#']]), 4);
    }
    test(max_students);
    test(max_students2);
}
