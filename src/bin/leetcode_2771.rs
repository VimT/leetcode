//! 构造最长非递减子数组

/// 贪心做法 （最好先想DP，再想贪心！）
pub fn max_non_decreasing_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let len = a.len();
    let mut result = 0;
    let mut i = 0;
    while i < len {
        let mut last = a[i].min(b[i]);
        let mut next = None;
        let mut j = i + 1;
        while j < len {
            let (aa, bb) = if a[j] > b[j] { (b[j], a[j]) } else { (a[j], b[j]) };
            if aa >= last {
                last = aa;
                next = None;
            } else if bb >= last {
                last = bb;
                if next.is_none() { next = Some(j); } // 当以b为当前子数组的值的时候，可能以a会更优，所以下次从a开始遍历
            } else {
                break;
            }
            j += 1;
        }
        result = result.max(j - i);
        i = next.unwrap_or(j);
    }

    result as i32
}

/// dp[i][0/1] 表示以i结尾的最后一个数是nums1/nums2的最大长度
pub fn max_non_decreasing_length2(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let len = a.len();
    let mut dp = vec![vec![1; 2]; len];
    let mut result = 1;
    for i in 1..len {
        if a[i] >= a[i - 1] { dp[i][0] = dp[i][0].max(dp[i - 1][0] + 1); }
        if a[i] >= b[i - 1] { dp[i][0] = dp[i][0].max(dp[i - 1][1] + 1); }
        if b[i] >= a[i - 1] { dp[i][1] = dp[i][1].max(dp[i - 1][0] + 1); }
        if b[i] >= b[i - 1] { dp[i][1] = dp[i][1].max(dp[i - 1][1] + 1); }
        result = result.max(dp[i][0]).max(dp[i][1]);
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![19, 3, 18, 5, 12], vec![5, 17, 3, 14, 16]), 4);
        assert_eq!(func(vec![12, 10], vec![16, 2]), 1);
        assert_eq!(func(vec![2, 3, 1], vec![1, 2, 1]), 2);
        assert_eq!(func(vec![1, 3, 2, 1], vec![2, 2, 3, 4]), 4);
        assert_eq!(func(vec![1, 1], vec![2, 2]), 2);
    }
    test(max_non_decreasing_length);
    test(max_non_decreasing_length2);
}
