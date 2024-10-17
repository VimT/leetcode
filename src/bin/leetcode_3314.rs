//! 构造最小位运算数组 I

pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().map(|num| {
        return if num == 2 { -1 } else {
            // 连续的 1 的最后一位变成 0
            let mut p = 0;
            while num >> p & 1 == 1 { p += 1; }
            num ^ (1 << (p - 1))
        };
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 3, 5, 7]), vec![-1, 1, 4, 3]);
        assert_eq!(func(vec![11, 13, 31]), vec![9, 12, 15]);
    }
    test(min_bitwise_array);
}
