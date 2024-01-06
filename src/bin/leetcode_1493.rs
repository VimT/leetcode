//! 删掉一个元素以后全为 1 的最长子数组

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    let mut used = 0;
    let mut unused = 0;
    for i in 0..len {
        if nums[i] == 0 {
            used = unused;
            unused = 0;
        } else {
            unused += 1;
            used += 1;
            result = result.max(used)
        }
    }
    result.min(len - 1) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1]), 11);
        assert_eq!(func(vec![1, 1, 0, 1]), 3);
        assert_eq!(func(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
        assert_eq!(func(vec![1, 1, 1]), 2);
    }
    test(longest_subarray);
}
