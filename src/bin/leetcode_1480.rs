//! 一维数组的动态和

pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(func(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(func(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
    test(running_sum);
}
