//! 最大连续1的个数 II

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut all_one = 0;
    let mut one_with_1zero = 0;
    let mut result = 0;
    for num in nums {
        if num == 1 {
            all_one += 1;
            one_with_1zero += 1;
        } else {
            one_with_1zero = all_one + 1;
            all_one = 0;
        }
        result = result.max(one_with_1zero);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 0, 1, 1, 0]), 4);
        assert_eq!(func(vec![1, 0, 1, 1, 0, 1]), 4);
    }
    test(find_max_consecutive_ones);
}
