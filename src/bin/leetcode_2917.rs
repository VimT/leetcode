//! 找出数组中的 K-or 值

pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt = vec![0; 32];
    for num in nums {
        for i in 0..32 {
            if num >> i & 1 == 1 {
                cnt[i] += 1;
            }
        }
    }
    let mut result = 0;
    for i in 0..32 {
        if cnt[i] >= k {
            result |= 1 << i;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![7, 12, 9, 8, 9, 15], 4), 9);
        assert_eq!(func(vec![2, 12, 1, 11, 4, 5], 6), 0);
        assert_eq!(func(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
    test(find_k_or);
}
