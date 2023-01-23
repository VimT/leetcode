//! 跳跃游戏 V

/// 记忆化搜索
pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    fn dfs(arr: &Vec<i32>, d: i32, i: usize, cache: &mut Vec<i32>) -> i32 {
        if cache[i] != -1 {
            return cache[i];
        }
        let mut result = 0;
        for left in ((i as i32 - d).max(0) as usize..i).rev() {
            if arr[left] >= arr[i] { break; }
            result = result.max(dfs(arr, d, left, cache));
        }
        for right in i + 1..(i + d as usize + 1 as usize).min(arr.len()) {
            if arr[right] >= arr[i] { break; }
            result = result.max(dfs(arr, d, right, cache));
        }
        cache[i] = result + 1;
        result + 1
    }
    let mut result = 0;
    let mut cache = vec![-1; arr.len()];
    for i in 0..arr.len() {
        result = result.max(dfs(&arr, d, i, &mut cache));
    }
    result
}

/// 记忆化搜索的dp版
pub fn max_jumps2(arr: Vec<i32>, d: i32) -> i32 {
    let len = arr.len();
    let mut sorted: Vec<(i32, usize)> = arr.iter().cloned().zip(0..).collect();
    sorted.sort_unstable();
    let mut dp = vec![1; len];
    for (max_height, i) in sorted {
        let left = (i as i32 - d..i as i32).rev().take_while(|&x| x >= 0 && arr[x as usize] < max_height).map(|x| dp[x as usize] + 1).max().unwrap_or(0);
        let right = (i + 1..(i + d as usize + 1).min(len)).take_while(|&x| arr[x] < max_height).map(|x| dp[x] + 1).max().unwrap_or(0);
        dp[i] = dp[i].max(left).max(right);
    }
    dp.into_iter().max().unwrap()
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, d: i32) -> i32) {
        assert_eq!(func(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
        assert_eq!(func(vec![3, 3, 3, 3, 3], 3), 1);
        assert_eq!(func(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    }
    test(max_jumps);
    test(max_jumps2);
}
