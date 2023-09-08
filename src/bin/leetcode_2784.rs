//! 检查数组是否是好的

pub fn is_good(mut nums: Vec<i32>) -> bool {
    let len = nums.len();
    nums.sort_unstable();
    for i in 0..len - 1 {
        if nums[i] != i as i32 + 1 {
            return false;
        }
    }
    nums[len - 1] == len as i32 - 1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![2, 1, 3]), false);
        assert_eq!(func(vec![1, 3, 3, 2]), true);
        assert_eq!(func(vec![1, 1]), true);
        assert_eq!(func(vec![3, 4, 4, 1, 2, 1]), false);
    }
    test(is_good);
}
