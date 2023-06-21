//! 找出分区值

pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut result = i32::MAX;
    for a in nums.windows(2) {
        result = result.min(a[1] - a[0]);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 2, 4]), 1);
        assert_eq!(func(vec![100, 1, 10]), 9);
    }
    test(find_value_of_partition);
}
