//! 判断是否能拆分数组


/// 脑筋急转弯，只要有相邻的 num 和 >= m 就行
pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
    if nums.len() <= 2 { return true; }
    for win in nums.windows(2) {
        if win[0] + win[1] >= m {
            return true;
        }
    }
    false
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, m: i32) -> bool) {
        assert_eq!(func(vec![2, 1, 3], 5), false);
        assert_eq!(func(vec![2, 2, 1], 4), true);
        assert_eq!(func(vec![2, 3, 3, 2, 3], 6), true);
    }
    test(can_split_array);
}
