//! 标记所有下标的最早秒数 I

pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
    fn check(nums: &Vec<i32>, change_indices: &[i32]) -> bool {
        let mut last_seen = vec![-1; nums.len() + 1];
        for (&num_i, second) in change_indices.iter().zip(1..) {
            last_seen[num_i as usize] = second;
        }
        for i in 1..=nums.len() {
            if last_seen[i] == -1 { return false; }  // 所有都要标记
        }
        let mut can_use = 0;
        let mut sum = nums.iter().map(|x| *x as i64).sum::<i64>();
        for (&num_i, second) in change_indices.iter().zip(1..) {
            if last_seen[num_i as usize] == second {
                // num_i 该下标是 change_indices 最后一次出现的 num_i
                let num = nums[num_i as usize - 1];
                if num > can_use {
                    return false;
                }
                can_use -= num;
                sum -= num as i64;
            } else {
                can_use += 1;
            }
            if sum == 0 { return true; }
        }
        false
    }

    if !check(&nums, &change_indices) { return -1; }

    let mut left = 0;
    let mut right = change_indices.len() + 1;
    while left < right {
        let mid = (left + right) / 2;
        if check(&nums, &change_indices[..mid]) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, change_indices: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1], vec![2, 2, 2]), -1);
        assert_eq!(func(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1]), 8);
        assert_eq!(func(vec![1, 3], vec![1, 1, 1, 2, 1, 1, 1]), 6);
    }
    test(earliest_second_to_mark_indices);
}
