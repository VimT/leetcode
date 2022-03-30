//! 最长递增子序列


pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    fn inner(nums: &Vec<i32>, i: usize, current_max: i32, current_count: i32) -> i32 {
        if i == nums.len() { return current_count; }
        if nums[i] > current_max {
            return inner(nums, i + 1, nums[i], current_count + 1).max(inner(nums, i + 1, current_max, current_count));
        }
        return inner(nums, i + 1, current_max, current_count).max(inner(nums, i + 1, nums[i], 1));
    }
    return inner(&nums, 1, nums[0], 1);
}

/// 末尾元素为nums[i]的最长上升序列， dp[i] = max(dp[j]) 0 <= j < i && nums[i] > nums[j]
pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];
    for i in 1..nums.len() {
        dp[i] = (0..i).filter(|j| nums[*j] < nums[i]).map(|x| dp[x]).fold(0, i32::max) + 1;
    }
    return *dp.last().unwrap();
}

/// 贪心 + 二分查找
/// d[i] ，表示长度为 i 的最长上升子序列的末尾元素的最小值
/// d[i] 是关于 i 单调递增的。
/// nums[i] > d[len], len++
/// nums[i] < d[len], 找 d[ x ] < nums[i]，更新d[x+1] = nums[i]
pub fn length_of_lis_greedy(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }
    let mut dp = vec![nums[0]];
    let mut len = 0;
    for i in nums {
        if i > dp[len] {
            dp.push(i);
            len += 1;
        } else {
            let k = dp.binary_search(&i).unwrap_or_else(|x| x);
            dp[k] = i;
        }
    }
    return dp.len() as i32;
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(func(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(func(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
    test(length_of_lis);
    test(length_of_lis_dp);
    test(length_of_lis_greedy);
}
