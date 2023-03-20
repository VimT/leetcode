//! 最大化数组的伟大值

pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut j = 0;
    let mut result = 0;
    for i in 0..len {
        while j < len && nums[j] <= nums[i] {
            j += 1;
        }
        if j == len { break; }
        j += 1;
        result += 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 0, 1, 0, 2]), 4);
        assert_eq!(func(vec![1, 3, 5, 2, 1, 3, 1]), 4);
        assert_eq!(func(vec![1, 2, 3, 4]), 3);
    }
    test(maximize_greatness);
}
