//! 通过操作使数组长度最小

/// 脑筋急转弯
pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
    let mn = *nums.iter().min().unwrap();
    for &num in &nums {
        if num % mn > 0 {
            return 1;
        }
    }
    (nums.into_iter().filter(|&x| x == mn).count() as i32 + 1) / 2
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 4, 3, 4, 1, 1, 1, 2]), 2);
        assert_eq!(func(vec![6, 9]), 1);
        assert_eq!(func(vec![1, 4, 3, 1]), 1);
        assert_eq!(func(vec![5, 5, 5, 10, 5]), 2);
        assert_eq!(func(vec![2, 3, 4]), 1);
    }
    test(minimum_array_length);
}
