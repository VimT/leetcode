//! 分割数组

pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut cnt = [0; 101];
    for num in nums {
        cnt[num as usize] += 1;
        if cnt[num as usize] > 2 { return false; }
    }
    true
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 1, 2, 2, 3, 4]), true);
        assert_eq!(func(vec![1, 1, 1, 1]), false);
    }
    test(is_possible_to_split);
}
