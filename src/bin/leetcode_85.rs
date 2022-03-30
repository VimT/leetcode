//! 最大矩形

use std::collections::VecDeque;

/// 思路为简化为上题的 柱状图的最大矩阵
/// 和上题的暴力法结合，有点难懂
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let rows = matrix.len();
    if rows == 0 { return 0; }
    let columns = matrix[0].len();
    let mut dp = vec![vec![0; columns]; rows];
    let mut ans = 0;
    for i in 0..rows {
        for j in 0..columns {
            if matrix[i][j] == '1' {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                    ans = ans.max(1);
                } else {
                    let mut width = dp[i][j - 1] + 1;
                    dp[i][j] = width;

                    for k in (0..=i).rev() {
                        width = width.min(dp[k][j]);
                        ans = ans.max(width * (i - k + 1));
                    }
                }
            }
        }
    }
    ans as i32
}

/// 同理，单调栈
/// 遍历每行的 1 高度，计算当前高度下的最大面积
pub fn maximal_rectangle_stack(matrix: Vec<Vec<char>>) -> i32 {
    fn leetcode_84(mut heights: Vec<i32>) -> i32 {
        let mut stack = VecDeque::new();
        let mut ans = 0;
        heights.push(0);
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.back().unwrap()] > heights[i] {
                let cur = stack.pop_back().unwrap();
                let left = stack.back().map_or(0, |x| *x + 1);
                let right = i - 1;
                ans = ans.max((right - left + 1) as i32 * heights[cur]);
            }
            stack.push_back(i);
        }
        ans
    }
    let mut ans = 0;
    let rows = matrix.len();
    if rows == 0 { return 0; }
    let columns = matrix[0].len();
    let mut dp = vec![0; columns];
    for i in 0..rows {
        for j in 0..columns {
            dp[j] = if matrix[i][j] == '1' { dp[j] + 1 } else { 0 };
        }
        ans = ans.max(leetcode_84(dp.clone()));
    }
    ans
}

/// 给定一个最大矩形，其高为 h， 左边界 l，右边界 r
/// 使用动态规划，我们可以在线性时间内用上一行每个点的 h，l，和 r 计算出下一行每个点的的h，l，和r
/// left, right, height 三个数组，遍历时更新
pub fn maximal_rectangle_dp(matrix: Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    let rows = matrix.len();
    if rows == 0 { return 0; }
    let columns = matrix[0].len();
    let mut left = vec![0; columns];
    let mut right = vec![columns; columns];
    let mut height = vec![0; columns];
    for i in 0..rows {
        let mut cur_left = 0;
        let mut cur_right = columns;
        for j in 0..columns {
            if matrix[i][j] == '1' { height[j] += 1; } else { height[j] = 0; }
        }
        for j in 0..columns {
            if matrix[i][j] == '1' {
                left[j] = left[j].max(cur_left);
            } else {
                left[j] = 0;
                cur_left = j + 1;
            }
        }
        for j in (0..columns).rev() {
            if matrix[i][j] == '1' {
                right[j] = right[j].min(cur_right);
            } else {
                right[j] = columns;
                cur_right = j;
            }
        }
        // println!("{:?},{:?},{:?}", left, right, height);
        for j in 0..columns {
            ans = ans.max(height[j] * (right[j] - left[j]));
        }
    }

    ans as i32
}

fn main() {
    fn test(func: fn(matrix: Vec<Vec<char>>) -> i32) {
        assert_eq!(func(vec![vec!['1']]), 1);
        assert_eq!(func(vec![vec!['1', '0', '1', '0', '0'], vec!['1', '0', '1', '1', '1'], vec!['1', '1', '1', '1', '1'], vec!['1', '0', '0', '1', '0']]), 6);
        assert_eq!(func(vec![vec!['0']]), 0);
    }
    test(maximal_rectangle);
    test(maximal_rectangle_dp);
    test(maximal_rectangle_stack);
}
