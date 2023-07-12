//! 所有子数组中不平衡数字之和


pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = nums.len();
    for i in 0..len - 1 {
        // 统计以i开头的所有子数组
        let mut seen = vec![false; 1002];
        seen[nums[i] as usize] = true;
        let mut cnt = 0;
        for &num in &nums[i + 1..len] {
            let x = num as usize;
            if !seen[x] {
                seen[x] = true;
                // 两边都没有出现，不平衡数+1，一边出现：不变，两边都有：不平衡数-1
                cnt += 1 - seen[x - 1] as i32 - seen[x + 1] as i32;
            }
            result += cnt;
        }
    }
    result
}

/// 贡献法：统计每个 nums[i] 有多少个包含 nums[i] ，不包含 nums[i-1] 的子数组。这些子数组里，只要 nums[i]不是最小值，就能产生贡献
pub fn sum_imbalance_numbers2(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut right = vec![0; len]; // nums[i] 右侧的 x 和 x-1 的最近下标
    let mut idx = vec![len; len + 1];
    for i in (0..len).rev() {
        let x = nums[i] as usize;
        right[i] = idx[x].min(idx[x - 1]);
        idx[x] = i;
    }
    let mut result = 0;
    let mut idx = vec![-1; len + 1];  // nums[i] 左侧包含x，不包含 x-1 的最近下标。巧妙的处理了重复贡献问题
    for ((x, r), i) in nums.into_iter().zip(right).zip(0..) {
        result += (i - idx[x as usize - 1] as i32) * (r as i32 - i);
        idx[x as usize] = i;
    }
    result - len as i32 * (len as i32 + 1) / 2  // 减去nums[i]作为最小值的子数组情况
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 4, 2]), 5);
        assert_eq!(func(vec![2, 3, 1, 4]), 3);
        assert_eq!(func(vec![1, 3, 3, 3, 5]), 8);
    }
    test(sum_imbalance_numbers);
    test(sum_imbalance_numbers2);
}
