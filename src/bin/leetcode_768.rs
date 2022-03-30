//! 最多能完成排序的块 II

use std::collections::HashMap;

/// 如果0－m是一个块，0－n是一个块（n>m) ，那么m+1..n也是一个块。
/// 如果排序后前k个字符和排序前前k个相同，则说明这是一个块
pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    let mut nonzero = 0;
    let mut sorted = arr.clone();
    sorted.sort_unstable();
    for (x, y) in arr.into_iter().zip(sorted) {
        *cnt.entry(x).or_default() += 1;
        if cnt[&x] == 0 { nonzero -= 1; } else if cnt[&x] == 1 { nonzero += 1; }
        *cnt.entry(y).or_default() -= 1;
        if cnt[&y] == -1 { nonzero += 1; } else if cnt[&y] == 0 { nonzero -= 1; }
        if nonzero == 0 { result += 1; }
    }
    result
}

/// 最小的 k 只要满足 max(arr[:k+1]) == expect[k] 就可以了，这时候 arr[:k+1] 一定是 expect[:k+1] 的某种排列组合。
pub fn max_chunks_to_sorted_max(arr: Vec<i32>) -> i32 {
    let mut cnt = HashMap::new();
    let counted: Vec<(i32, i32)> = arr.into_iter().map(|x| {
        *cnt.entry(x).or_insert(0i32) += 1;
        (x, cnt[&x])
    }).collect();
    let mut sorted = counted.clone();
    sorted.sort_unstable();
    let mut cur = (0, 0);
    let mut result = 0;
    for (x, y) in counted.into_iter().zip(sorted) {
        cur = cur.max(x);
        if cur == y {
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(max_chunks_to_sorted_max(vec![5, 4, 3, 2, 1]), 1);
    assert_eq!(max_chunks_to_sorted_max(vec![2, 1, 3, 4, 4]), 4);
}
