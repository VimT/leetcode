//! 将数组分割成最多数目的子数组

pub fn max_subarrays(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut cur = -1;  // 初始化一个二进制全1的数，就是-1
    for num in nums {
        cur &= num;
        if cur == 0 {
            result += 1;
            cur = -1;
        }
    }
    result.max(1)  // 如果没有分割过，返回1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 0, 2, 0, 1, 2]), 3);
        assert_eq!(func(vec![5, 7, 1, 3]), 1);
    }
    test(max_subarrays);
}
