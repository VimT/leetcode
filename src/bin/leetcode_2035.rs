//! 将数组分成两个数组并最小化数组和的差

/// 折半枚举+排序+二分
pub fn minimum_difference(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let n = len / 2;
    let mut s = vec![vec![]; n + 1];
    for i in 0..(1 << n) {
        let mut sum = 0;
        let mut cnt = 0;
        // 根据i的二进制位决定 前n个数，每个数的分组
        for j in 0..n {
            if i >> j & 1 == 1 {
                sum += nums[j];
                cnt += 1;
            } else {
                sum -= nums[j];
            }
        }
        s[cnt].push(sum);
    }
    for i in 0..n + 1 {
        s[i].sort_unstable();
    }
    let mut result = i32::MAX;
    for i in 0..(1 << n) {
        let mut sum = 0;
        let mut cnt = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                sum += nums[j + n];
                cnt += 1;
            } else {
                sum -= nums[j + n];
            }
        }
        let pos = s[cnt].binary_search(&sum).unwrap_or_else(|x| x);
        if pos < s[cnt].len() {
            result = result.min(s[cnt][pos] - sum);
        }
        if pos > 0 {
            result = result.min(sum - s[cnt][pos - 1]);
        }
    }
    result
}


fn main() {
    assert_eq!(minimum_difference(vec![3, 9, 7, 3]), 2);
    assert_eq!(minimum_difference(vec![-36, 36]), 72);
    assert_eq!(minimum_difference(vec![2, -1, 0, 4, -2, -9]), 0);
}
