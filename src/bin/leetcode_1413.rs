//! 逐步求和得到正数的最小值

pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut min = 0;
    let mut cur_sum = 0;
    for num in nums {
        cur_sum += num;
        min = min.min(cur_sum);
    }
    -min + 1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(func(vec![1, 2]), 1);
        assert_eq!(func(vec![1, -2, -3]), 5);
    }
    test(min_start_value);
}
