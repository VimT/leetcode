//! 三个无重叠子数组的最大和

pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    let k = k as usize;
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i];
    }
    let mut prefix = vec![(0, 0); len - 2 * k];
    let mut suffix = vec![(0, 0); len - 2 * k];
    let mut max = 0;
    let mut max_idx = 0;
    for i in k..=len - 2 * k {
        if presum[i] - presum[i - k] > max {
            max = presum[i] - presum[i - k];
            max_idx = i;
        }
        prefix[i - k] = (max, max_idx - k);
    }
    max = 0;
    max_idx = 0;
    for i in (3 * k..=len).rev() {
        if presum[i] - presum[i - k] >= max {
            max = presum[i] - presum[i - k];
            max_idx = i;
        }
        suffix[i - 3 * k] = (max, max_idx - k);
    }
    max = 0;
    let mut result = vec![0, 0, 0];
    for i in k..=len - 2 * k {
        if prefix[i - k].0 + suffix[i - k].0 + presum[i + k] - presum[i] > max {
            max = prefix[i - k].0 + suffix[i - k].0 + presum[i + k] - presum[i];
            result = vec![prefix[i - k].1, i, suffix[i - k].1];
        }
    }
    result.into_iter().map(|x| x as i32).collect()
}

/// 三个大小为 k 的滑动窗口
pub fn max_sum_of_three_subarrays_multi_win(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = vec![];
    let (mut sum1, mut max_sum1, mut max_sum1_idx) = (0, 0, 0);
    let (mut sum2, mut max_sum12, mut max_sum12_idx) = (0, 0, (0, 0));
    let (mut sum3, mut max_total) = (0, 0);
    let k = k as usize;
    let len = nums.len();
    for i in 2 * k..len {
        sum1 += nums[i - k * 2];
        sum2 += nums[i - k];
        sum3 += nums[i];
        if i >= 3 * k - 1 {
            if sum1 > max_sum1 {
                max_sum1 = sum1;
                max_sum1_idx = i + 1 - 3 * k;
            }
            if max_sum1 + sum2 > max_sum12 {
                max_sum12 = max_sum1 + sum2;
                max_sum12_idx = (max_sum1_idx, i + 1 - 2 * k);
            }
            if max_sum12 + sum3 > max_total {
                max_total = max_sum12 + sum3;
                result = vec![max_sum12_idx.0 as i32, max_sum12_idx.1 as i32, (i + 1 - k) as i32];
            }
            sum1 -= nums[i + 1 - k * 3];
            sum2 -= nums[i + 1 - k * 2];
            sum3 -= nums[i + 1 - k];
        }
    }
    result
}

fn main() {
    assert_eq!(max_sum_of_three_subarrays_multi_win(vec![1, 2, 1, 2, 6, 7, 5, 1], 2), [0, 3, 5]);
    assert_eq!(max_sum_of_three_subarrays_multi_win(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2), [0, 2, 4]);
}
