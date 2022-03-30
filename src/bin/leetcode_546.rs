//! 移除盒子

use std::collections::HashMap;

pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
    fn dfs(boxes: Vec<(i32, i32)>, cache: &mut HashMap<Vec<(i32, i32)>, i32>) -> i32 {
        if let Some(v) = cache.get(&boxes) {
            return *v;
        }
        if boxes.len() == 0 { return 0; }
        let len = boxes.len();
        let (num, cnt) = boxes[0];
        let mut result = dfs(boxes[1..].to_vec(), cache) + (cnt * cnt);
        for i in 1..len {
            if boxes[i].0 == num {
                let middle = dfs(boxes[1..i].to_vec(), cache);
                let mut left = Vec::with_capacity(1 + len - i);
                left.push((num, cnt + boxes[i].1));
                left.extend_from_slice(&boxes[i + 1..]);
                result = result.max(middle + dfs(left, cache));
            }
        }
        cache.insert(boxes, result);
        result as i32
    }
    let mut b = vec![];
    let mut start = 0;
    let len = boxes.len();
    while start < len {
        let mut end = start + 1;
        while end < len && boxes[end] == boxes[start] { end += 1; }
        b.push((boxes[start], (end - start) as i32));
        start = end;
    }
    dfs(b, &mut HashMap::new())
}


/// dp(l,r,k) 表示移除区间 [l, r] 的元素  加上该区间右边等于 a[r]  的 k 个元素组成的这个序列的最大积分
pub fn remove_boxes_dp(boxes: Vec<i32>) -> i32 {
    let len = boxes.len();
    let mut dp = vec![vec![vec![0; len]; len]; len];
    fn dfs(boxes: &Vec<i32>, l: usize, r: usize, k: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if l > r { return 0; }
        if dp[l][r][k] == 0 {
            dp[l][r][k] = if r > 0 { dfs(boxes, l, r - 1, 0, dp) } else { 0 } + ((k + 1) * (k + 1)) as i32;
            for i in l..r {
                if boxes[i] == boxes[r] {
                    dp[l][r][k] = dp[l][r][k].max(dfs(boxes, l, i, k + 1, dp) + dfs(boxes, i + 1, r - 1, 0, dp));
                }
            }
        }
        dp[l][r][k]
    }
    dfs(&boxes, 0, boxes.len() - 1, 0, &mut dp)
}

fn main() {
    assert_eq!(remove_boxes_dp(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
    assert_eq!(remove_boxes_dp(vec![1, 1, 1]), 9);
    assert_eq!(remove_boxes_dp(vec![1]), 1);
}
