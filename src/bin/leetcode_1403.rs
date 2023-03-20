//! 非递增顺序的最小子序列

pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
    let sum: i32 = nums.iter().sum();
    nums.sort_unstable();
    nums.reverse();
    let mut cur = 0;
    for i in 0..nums.len() {
        cur += nums[i];
        if cur > sum - cur {
            return nums[..=i].to_vec();
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![4, 3, 10, 9, 8]), vec![10, 9]);
        assert_eq!(func(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    }
    test(min_subsequence);
}
