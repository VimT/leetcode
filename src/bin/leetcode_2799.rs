//! 统计完全子数组的数目

use std::collections::HashSet;

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    let cnt: HashSet<i32> = nums.iter().copied().collect();
    let mut result = 0;
    let len = nums.len();
    for i in 0..len {
        let mut c2 = HashSet::new();
        for j in i..len {
            c2.insert(nums[j]);
            if c2.len() == cnt.len() {
                result += 1;
            }
        }
    }
    result
}

/// 滑动窗口 O(n)
pub fn count_complete_subarrays2(nums: Vec<i32>) -> i32 {
    let mut cnt = vec![0; 2001];
    let mut total = 0;
    for &num in &nums {
        if cnt[num as usize] == 0 {
            total += 1;
        }
        cnt[num as usize] = 1;
    }

    cnt.fill(0);
    let mut result = 0;
    let len = nums.len();
    let mut cur = 0;
    let mut l = 0;
    for r in 0..len {
        if cnt[nums[r] as usize] == 0 { cur += 1; }
        cnt[nums[r] as usize] += 1;
        if cur == total {
            while cnt[nums[l] as usize] > 1 {
                cnt[nums[l] as usize] -= 1;
                l += 1;
            }
            result += l as i32 + 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 1, 2, 2]), 4);
        assert_eq!(func(vec![5, 5, 5, 5]), 10);
    }
    test(count_complete_subarrays);
    test(count_complete_subarrays2);
}
