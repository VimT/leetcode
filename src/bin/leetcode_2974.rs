//! 最小数字游戏

pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    for i in (1..nums.len()).step_by(2) {
        nums.swap(i, i - 1);
    }
    nums
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
        assert_eq!(func(vec![2, 5]), vec![5, 2]);
    }
    test(number_game);
}
