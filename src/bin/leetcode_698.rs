//! 划分为k个相等的子集


pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
    fn dfs(nums: &Vec<i32>, status: i32, mut k: i32, mut cur_sum: i32, target: i32, cache: &mut Vec<bool>) -> bool {
        let len = nums.len();
        if cur_sum == target {
            cur_sum = 0;
            k -= 1;
        }
        if cache[status as usize] { return false; }
        if k == 0 {
            return true;
        }
        for i in 0..len {
            if status >> i & 1 == 0 {
                if cur_sum + nums[i] <= target {
                    if dfs(nums, status ^ 1 << i, k, cur_sum + nums[i], target, cache) {
                        return true;
                    }
                }
            }
        }
        cache[status as usize] = true;
        false
    }
    let sum: i32 = nums.iter().sum();
    if sum % k != 0 { return false; }
    nums.sort_unstable_by_key(|x| -x);
    let target = sum / k;
    dfs(&nums, 0, k, 0, target, &mut vec![false; 1 << nums.len()])
}

pub fn can_partition_k_subsets_dp(mut nums: Vec<i32>, k: i32) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % k != 0 { return false; }
    nums.sort_unstable_by_key(|x| -x);
    let target = sum / k;
    if nums[0] > target { return false; }
    let len = nums.len();
    let mut dp = vec![false; 1 << len];
    let mut cur_sum = vec![0; 1 << len];
    dp[0] = true;
    for i in 0..1 << len {
        if !dp[i] { continue; }
        for j in 0..len {
            if i >> j & 1 == 1 { continue; }
            let next = i | (1 << j);
            if dp[next] { continue; }
            // 用cur_sum[i] 表示i状态的所有数字和， cur_sum[i] % target 就是对应dfs的 cur_sum和k。。
            if (cur_sum[i] % target) + nums[j] <= target {
                cur_sum[next] = cur_sum[i] + nums[j];
                dp[next] = true;
            } else {
                continue;
            }
        }
    }
    dp[(1 << len) - 1]
}

fn main() {
    assert_eq!(can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4), false);
    assert_eq!(can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
    assert_eq!(can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
}
