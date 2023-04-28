//! 一个数组所有前缀的分数

pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
    let mut cur_max = 0;
    let mut cur_sum = 0;
    let mut result = Vec::with_capacity(nums.len());
    for num in nums {
        cur_max = cur_max.max(num);
        cur_sum += num as i64 + cur_max as i64;
        result.push(cur_sum);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i64>) {
        assert_eq!(func(vec![2, 3, 7, 5, 10]), vec![4, 10, 24, 36, 56]);
        assert_eq!(func(vec![1, 1, 2, 4, 8, 16]), vec![2, 4, 8, 16, 32, 64]);
    }
    test(find_prefix_score);
}
