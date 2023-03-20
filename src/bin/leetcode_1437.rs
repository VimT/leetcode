//! 是否所有 1 都至少相隔 k 个元素

pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut last1 = -1;
    for (i, num) in nums.into_iter().enumerate() {
        if num == 1 {
            if last1 != -1 && i as i32 - last1 - 1 < k {
                return false;
            }
            last1 = i as i32;
        }
    }
    true
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> bool) {
        assert_eq!(func(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
        assert_eq!(func(vec![1, 0, 0, 1, 0, 1], 2), false);
        assert_eq!(func(vec![1, 1, 1, 1, 1], 0), true);
    }
    test(k_length_apart);
}
