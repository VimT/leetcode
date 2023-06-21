//! 好数对的数目

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut cnt = vec![0; 101];
    for num in nums {
        cnt[num as usize] += 1;
    }
    let mut result = 0;
    for i in 0..101 {
        if cnt[i] > 1 {
            result += cnt[i] * (cnt[i] - 1) / 2;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(func(vec![1, 1, 1, 1]), 6);
        assert_eq!(func(vec![1, 2, 3]), 0);
    }
    test(num_identical_pairs);
}
