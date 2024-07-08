//! 找出有效子序列的最大长度 II

/// 寻找一个最长的子序列，满足子序列奇数项都相同，偶数项都相同
pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let nums: Vec<i32> = nums.into_iter().map(|x| x % k).collect();
    let len = nums.len();
    let mut dp = vec![vec![0; k as usize]; len]; // dp[i][j] 表示以i结尾，余数为 j 最长有效子序列的长度
    let mut result = 0;
    for i in 1..len {
        for j in 0..i {
            let x = ((nums[i] + nums[j]) % k) as usize;
            dp[i][x] = dp[j][x] + 1;
            result = result.max(dp[i][x]);
        }
    }
    result + 1
}

/// 二维数组 f[y][x]，表示最后两项模 k 分别为 y 和 x 的子序列的长度。
/// f[y][x] = f[x][y] + 1
pub fn maximum_length2(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut f = vec![vec![0; k]; k];
    for num in nums {
        let x = num as usize % k;
        for y in 0..k {
            f[y][x] = f[x][y] + 1;
        }
    }
    f.into_iter().flatten().max().unwrap()
}

/// 枚举余数
/// 指定余数是 m 时，假设子序列的最后一项是 x ，那么倒数第二项是 (m - x%k) % k
pub fn maximum_length3(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    let k = k as usize;
    for m in 0..k {
        let mut f = vec![0; k];
        for &num in &nums {
            let x = num as usize % k;
            f[x] = f[(k + m - x) % k] + 1;
        }
        result = result.max(f.into_iter().max().unwrap());
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 5, 9, 2, 8], 2), 3);
        assert_eq!(func(vec![1, 2, 3, 4, 5], 2), 5);
        assert_eq!(func(vec![1, 4, 2, 3, 1, 4], 3), 4);
    }
    test(maximum_length);
    test(maximum_length2);
    test(maximum_length3);
}
