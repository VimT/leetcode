//! 最小操作次数使数组元素相等

pub fn min_moves(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    nums.into_iter().map(|x| x - min).sum()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3]), 3);
        assert_eq!(func(vec![1, 1, 1]), 0);
    }
    test(min_moves);
}
