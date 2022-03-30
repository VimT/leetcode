//! 数组的最大与和

/// 三进制状态压缩，dp[mask] 表示当前 篮子状态 （0：没装，1：装一个，2：装两个） 的最大与和
/// 遍历所有篮子状态，对于下一个要装的数字nums[cnt]， dp[mask] = max(dp[mask], dp[mask - 3^i] + nums[cnt] & (i+1))
pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
    let len = nums.len();
    let mut mask_max = 1;
    for _ in 0..num_slots {
        mask_max *= 3;
    }
    let mut dp = vec![0; mask_max];
    for mask in 1..mask_max {
        let mut cnt = 0;
        let mut dummy = mask;
        for _ in 0..num_slots {
            cnt += dummy % 3;
            dummy /= 3;
        }
        if cnt > len {
            continue;
        }
        let mut w = 1;
        dummy = mask;
        for i in 0..num_slots {
            if dummy % 3 != 0 {
                dp[mask] = dp[mask].max(dp[mask - w] + (nums[cnt - 1] & (i + 1)));
            }
            w *= 3;
            dummy /= 3;
        }
    }
    *dp.iter().max().unwrap()
}

fn main() {
    assert_eq!(maximum_and_sum(vec![1, 2, 3, 4, 5, 6], 3), 9);
    assert_eq!(maximum_and_sum(vec![1, 3, 10, 4, 7, 1], 9), 24);
}
