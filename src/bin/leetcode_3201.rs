//! 找出有效子序列的最大长度 I

/// 全0，全1，交替0和1的最大长度
pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let nums: Vec<i32> = nums.into_iter().map(|x| x % 2).collect();
    let mut cnt = [0; 2];
    let mut last = -1;
    let mut diff = 0;
    for num in nums {
        cnt[num as usize] += 1;
        if num != last {
            diff += 1;
        }
        last = num;
    }
    cnt[0].max(cnt[1]).max(diff)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 5, 9, 4, 2]), 3);
        assert_eq!(func(vec![1, 2, 3, 4]), 4);
        assert_eq!(func(vec![1, 2, 1, 1, 2, 1, 2]), 6);
        assert_eq!(func(vec![1, 3]), 2);
    }
    test(maximum_length);
}
