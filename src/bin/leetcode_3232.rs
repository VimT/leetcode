//! 判断是否可以赢得数字游戏

pub fn can_alice_win(nums: Vec<i32>) -> bool {
    nums.into_iter().map(|num| {
        if num < 10 { -num } else { num }
    }).sum::<i32>() != 0
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 4, 10]), false);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 14]), true);
        assert_eq!(func(vec![5, 5, 5, 25]), true);
    }
    test(can_alice_win);
}
