//! 将数组分成最小总代价的子数组 I

pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
    nums[1..].sort_unstable();
    nums[..3].iter().sum()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 12]), 6);
        assert_eq!(func(vec![5, 4, 3]), 12);
        assert_eq!(func(vec![10, 3, 1, 1]), 12);
    }
    test(minimum_cost);
}
