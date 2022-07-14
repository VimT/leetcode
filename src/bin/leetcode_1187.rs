//! 使数组严格递增

/// dp[i] 表示以 arr1[i] 结尾，没有替换时的最小操作数
pub fn make_array_increasing(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
    arr2.sort_unstable();
    let mut narr2 = Vec::with_capacity(arr2.len());
    for i in 0..arr2.len() {
        if i == 0 || arr2[i] != arr2[i - 1] {
            narr2.push(arr2[i]);
        }
    }
    arr2 = narr2;
    arr1.insert(0, -1);
    arr1.push(i32::MAX);
    let len = arr1.len();
    let mut dp = vec![i32::MAX / 2; len];
    dp[0] = 0;
    for i in 1..len {
        let j = arr2.binary_search(&arr1[i]).unwrap_or_else(|x| x);
        // 枚举替换个数
        for k in 1..=j.min(i - 1) {
            if arr1[i - k - 1] < arr2[j - k] {
                dp[i] = dp[i].min(dp[i - k - 1] + k as i32);
            }
        }
        // 不替换
        if arr1[i - 1] < arr1[i] {
            dp[i] = dp[i].min(dp[i - 1]);
        }
    }
    if dp[len - 1] >= i32::MAX / 2 { -1 } else { dp[len - 1] }
}

fn main() {
    fn test(func: fn(arr1: Vec<i32>, arr2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]), 1);
        assert_eq!(func(vec![1, 5, 3, 6, 7], vec![4, 3, 1]), 2);
        assert_eq!(func(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]), -1);
    }
    test(make_array_increasing);
}
