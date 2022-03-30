//! 最接近目标值的子序列和

pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
    fn get_sub_array_sum(nums: &[i32]) -> Vec<i32> {
        let len = 1 << nums.len();
        let mut s = vec![0; len];
        for i in 0..len {
            let mut sum = 0;
            for j in 0..nums.len() {
                if i >> j & 1 == 1 {
                    sum += nums[j];
                }
            }
            s[i] = sum;
        }
        s.sort_unstable();
        s
    }
    let mid = nums.len() / 2;
    let one = get_sub_array_sum(&nums[0..mid]);
    let two = get_sub_array_sum(&nums[mid..]);
    let mut left = 0;
    let mut right = two.len() - 1;
    let mut result = i32::MAX;
    while left < one.len() {
        let sum = one[left] + two[right];
        result = result.min((sum - goal).abs());
        if result == 0 {
            return result;
        } else if sum > goal {
            if right == 0 { break; }
            right -= 1;
        } else {
            left += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(min_abs_difference(vec![5, -7, 3, 5], 6), 0);
    assert_eq!(min_abs_difference(vec![7, -9, 15, -2], -5), 1);
    assert_eq!(min_abs_difference(vec![1, 2, 3], -7), 7);
}
