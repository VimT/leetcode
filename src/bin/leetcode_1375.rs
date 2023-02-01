//! 二进制字符串前缀一致的次数

pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut sum = 0;
    let mut result = 0;
    for i in flips {
        sum += 1;
        max = max.max(i);
        if sum == max {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(flips: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 4, 1, 5]), 2);
        assert_eq!(func(vec![4, 1, 2, 3]), 1);
    }
    test(num_times_all_blue);
}
