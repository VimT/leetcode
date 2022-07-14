//! 小于 K 的两数之和

pub fn two_sum_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut result = -1;
    for i in 0..len {
        for j in 0..len {
            let sum = nums[i] + nums[j];
            if sum < k && sum > result {
                result = sum;
            }
        }
    }
    result
}

pub fn two_sum_less_than_k2(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut result = -1;
    let len = nums.len();
    let mut j = len;
    for i in 0..len {
        while j > i + 1 && nums[i] + nums[j - 1] >= k {
            j -= 1;
        }
        if j > i + 1 {
            result = result.max(nums[i] + nums[j - 1]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![34, 23, 1, 24, 75, 33, 54, 8], 60), 58);
        assert_eq!(func(vec![10, 20, 30], 15), -1);
    }
    test(two_sum_less_than_k);
    test(two_sum_less_than_k2);
}
