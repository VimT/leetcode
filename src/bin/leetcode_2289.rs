//! 使数组按非递减顺序排列

/// 倒叙 单调碱栈，维护这个数入栈，删了几轮
pub fn total_steps(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut s: Vec<(i32, i32)> = vec![];
    let mut result = 0;
    for i in (0..len).rev() {
        let mut cnt = 0;
        while !s.is_empty() && s.last().unwrap().0 < nums[i] {
            cnt = (cnt + 1).max(s.pop().unwrap().1);
            result.max(cnt);
        }
        s.push((nums[i], cnt));
    }
    s.iter().map(|x| x.1).max().unwrap()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3);
        assert_eq!(func(vec![4, 5, 7, 7, 13]), 0);
        assert_eq!(func(vec![7, 14, 4, 14, 13, 2, 6, 13]), 3);
    }
    test(total_steps);
}
