//! 使二进制数组全部等于 1 的最少操作次数 I

pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = nums.len();
    for i in 0..len - 2 {
        if nums[i] == 0 {
            nums[i] ^= 1;
            nums[i + 1] ^= 1;
            nums[i + 2] ^= 1;
            result += 1;
        }
    }
    if nums[len - 2] == 1 && nums[len - 1] == 1 { result } else { -1 }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(func(vec![0, 1, 1, 1]), -1);
    }
    test(min_operations);
}
