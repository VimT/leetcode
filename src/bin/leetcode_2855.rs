//! 使数组成为递增数组的最少右移次数

pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    for i in 0..len {
        let mut n = vec![];
        n.extend_from_slice(&nums[len - i..len]);
        n.extend_from_slice(&nums[0..len - i]);
        if n.windows(2).all(|w| w[0] <= w[1]) {
            return i as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![31, 72, 79, 25]), 1);
        assert_eq!(func(vec![3, 4, 5, 1, 2]), 2);
        assert_eq!(func(vec![1, 3, 5]), 0);
        assert_eq!(func(vec![2, 1, 4]), -1);
    }
    test(minimum_right_shifts);
}
