//! 3n 块披萨

use std::collections::BinaryHeap;

/// dp[i][j] 表示在前 i 个数中选择了 j 个不相邻的数的最大和
/// dp[i][j] = (dp[i-2][j-1] + slice[i]).max(dp[i-2][j-1])
pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    fn inner(slices: &[i32]) -> i32 {
        let len = slices.len();
        let choose = (len + 1) / 3;
        let mut dp = vec![vec![0; choose + 1]; len + 1];
        for i in 1..=len {
            for j in 1..=choose {
                dp[i][j] = dp[i - 1][j].max(if i >= 2 { dp[i - 2][j - 1] } else { 0 } + slices[i - 1]);
            }
        }
        dp[len][choose]
    }
    inner(&slices[1..]).max(inner(&slices[..slices.len() - 1]))
}

/// 贪心算法
/// 6666666  should learn
pub fn max_size_slices_2(mut slices: Vec<i32>) -> i32 {
    let len = slices.len();
    // 数组实现链表
    let mut linkl = vec![0; len];
    let mut linkr = vec![0; len];
    for i in 0..len {
        linkl[i] = if i == 0 { len - 1 } else { i - 1 };
        linkr[i] = if i == len - 1 { 0 } else { i + 1 };
    }
    let mut valid = vec![true; len];
    let mut q = BinaryHeap::new();
    for i in 0..len {
        q.push((slices[i], i));
    }
    let mut result = 0;
    for _ in 0..len / 3 {
        while !valid[q.peek().unwrap().1] {
            q.pop();
        }
        let (value, idx) = q.pop().unwrap();
        result += value;
        // 后悔的方法: 8,9,8 -> 先选9，则留下一个7，如果7选上了，相当于选了8+8
        slices[idx] = slices[linkl[idx]] + slices[linkr[idx]] - slices[idx];
        q.push((slices[idx], idx));
        valid[linkl[idx]] = false;
        valid[linkr[idx]] = false;
        // 数组链表的维护
        linkr[linkl[linkl[idx]]] = idx;
        linkl[linkr[linkr[idx]]] = idx;
        linkl[idx] = linkl[linkl[idx]];
        linkr[idx] = linkr[linkr[idx]];
    }
    result
}


fn main() {
    assert_eq!(max_size_slices_2(vec![1, 2, 3, 4, 5, 6]), 10);
    assert_eq!(max_size_slices_2(vec![8, 9, 8, 6, 1, 1]), 16);
    assert_eq!(max_size_slices_2(vec![4, 1, 2, 5, 8, 3, 1, 9, 7]), 21);
    assert_eq!(max_size_slices_2(vec![3, 1, 2]), 3);
}
