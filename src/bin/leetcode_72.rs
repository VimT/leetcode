//! 编辑距离

//! 参考 https://leetcode-cn.com/problems/edit-distance/solution/dong-tai-gui-hua-di-tui-tian-biao-vs-ji-yi-hua-di-/
use std::collections::{HashSet, VecDeque};

/// 自顶而下。递归
/// dp[i][j] 表示 w1的前i个字符 和w2的前j个字符的编辑距离
pub fn min_distance(word1: String, word2: String) -> i32 {
    fn inner(w1: &[u8], w2: &[u8], i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] > 0 {
            return dp[i][j];
        }

        dp[i][j] = if i == 0 || j == 0 {
            (i + j) as i32
        } else {
            if w1[i - 1] == w2[j - 1] {
                inner(w1, w2, i - 1, j - 1, dp)
            } else {
                1 + inner(w1, w2, i - 1, j - 1, dp)
                    .min(inner(w1, w2, i - 1, j, dp))
                    .min(inner(w1, w2, i, j - 1, dp))
            }
        };
        dp[i][j]
    }
    let m = word1.len();
    let n = word2.len();
    inner(word1.as_bytes(), word2.as_bytes(), m, n, &mut vec![vec![-1; n + 1]; m + 1])
}

/// 自底而上
pub fn min_distance_dp(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let mut dp = vec![vec![-1; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i as i32; }
    for i in 0..=n { dp[0][i] = i as i32; }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if w1[i - 1] == w2[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j])
            }
        }
    }
    dp[m][n]
}

/// 空间复杂度优化
pub fn min_distance_dp_optimise(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let mut dp = vec![0; n + 1];
    for i in 0..=n { dp[i] = i as i32; }

    for i in 1..=m {
        let mut prev = dp[0];
        dp[0] = i as i32;
        for j in 1..=n {
            let curr = dp[j];
            dp[j] = if w1[i - 1] == w2[j - 1] {
                prev
            } else {
                1 + dp[j - 1].min(dp[j]).min(prev)
            };
            prev = curr;
        }
    }
    dp[n]
}

/// 广度优先搜索
/// 广度优先搜素需要将两个单词从最开始的字符进行比较。
/// 当出现不同字符时，说明要多编辑一次，编辑距离加一，广度搜素需要下探一层，下一层要比较的单词就是经过一次增删改之后的单词。
/// 当第一次扫到两个单词相同时，就输出最小编辑层数。加入访问记录可以帮助剪枝。
pub fn min_distance_bfs(word1: String, word2: String) -> i32 {
    let mut visited = HashSet::new();
    let mut q: VecDeque<(&[u8], &[u8], i32)> = VecDeque::new();
    q.push_back((word1.as_bytes(), word2.as_bytes(), 0));
    loop {
        let (mut w1, mut w2, mut d) = q.pop_front().unwrap();
        if visited.contains(&(w1, w2)) {
            if w1 == w2 { return d; }
        }
        visited.insert((w1, w2));
        while w1.len() > 0 && w2.len() > 0 && w1[0] == w2[0] {
            w1 = &w1[1..];
            w2 = &w2[1..];
        }
        d += 1;
        let nw1 = if w1.len() > 0 { &w1[1..] } else { b"" };
        let nw2 = if w2.len() > 0 { &w2[1..] } else { b"" };
        q.push_back((nw1, nw2, d));
        q.push_back((w1, nw2, d));
        q.push_back((nw1, w2, d));
    }
}


fn main() {
    fn test(func: fn(word1: String, word2: String) -> i32) {
        assert_eq!(func(String::from("horse"), String::from("ros")), 3);
        assert_eq!(func(String::from("intention"), String::from("execution")), 5);
    }
    test(min_distance);
    test(min_distance_bfs);
    test(min_distance_dp);
    test(min_distance_dp_optimise);
}
