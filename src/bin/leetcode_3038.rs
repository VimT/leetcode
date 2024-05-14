//! 相同分数的最大操作数目 I

pub fn max_operations(nums: Vec<i32>) -> i32 {
    let target = nums[0] + nums[1];
    let len = nums.len();
    for i in (3..len).step_by(2) {
        if nums[i - 1] + nums[i] != target {
            return i as i32 / 2;
        }
    }
    len as i32 / 2
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(func(vec![3, 2, 6, 1, 4]), 1);
    }
    test(max_operations);
}
