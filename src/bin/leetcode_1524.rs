//! 和为奇数的子数组数目

/// dp[i] = (odd, even) 表示以i结尾的子数组数，有odd个和为奇数的子数组，even个和为偶数的子数组
pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut even = 0;
    let mut odd = 0;
    let mut result = 0;
    const MOD: i32 = 1e9 as i32 + 7;
    for num in arr {
        if num & 1 == 1 {
            let tmp = odd;
            odd = even + 1;
            even = tmp;
        } else {
            even += 1;
        }
        result = (result + odd) % MOD;
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 5]), 4);
        assert_eq!(func(vec![2, 4, 6]), 0);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6, 7]), 16);
        assert_eq!(func(vec![100, 100, 99, 99]), 4);
        assert_eq!(func(vec![7]), 1);
    }
    test(num_of_subarrays);
}
