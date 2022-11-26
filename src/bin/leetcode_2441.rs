//! 与对应负数同时存在的最大正整数

pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by_key(|x| x.abs());
    for ab in nums.windows(2).rev() {
        if ab[0] == -ab[1] {
            return ab[0].abs();
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![-1, 2, -3, 3]), 3);
        assert_eq!(func(vec![-1, 10, 6, 7, -7, 1]), 7);
        assert_eq!(func(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
    test(find_max_k);
}
