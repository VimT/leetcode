//! 按位与结果大于零的最长组合

pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut cnt = [0; 32];
    for candidate in candidates {
        for i in 0..24 {
            if candidate >> i & 1 == 1 {
                cnt[i] += 1;
            }
        }
    }
    *cnt.iter().max().unwrap()
}

fn main() {
    fn test(func: fn(candidates: Vec<i32>) -> i32) {
        assert_eq!(func(vec![16, 17, 71, 62, 12, 24, 14]), 4);
        assert_eq!(func(vec![8, 8]), 2);
    }
    test(largest_combination);
}
