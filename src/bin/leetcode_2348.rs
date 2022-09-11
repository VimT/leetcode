//! 全 0 子数组的数目

pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut zero_cnt = 0;
    let mut result = 0;
    for num in nums {
        if num == 0 {
            zero_cnt += 1;
            result += zero_cnt as i64;
        } else {
            zero_cnt = 0;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1, 3, 0, 0, 2, 0, 0, 4]), 6);
        assert_eq!(func(vec![0, 0, 0, 2, 0, 0]), 9);
        assert_eq!(func(vec![2, 10, 2019]), 0);
    }
    test(zero_filled_subarray);
}
