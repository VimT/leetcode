//! 求出出现两次数字的 XOR 值

pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut m = [0; 51];
    for num in nums {
        m[num as usize] += 1;
    }
    let mut result = 0;
    for i in 0..=50 {
        if m[i] == 2 {
            result ^= i;
        }
    }
    result as i32
}

/// 一次遍历，位运算
pub fn duplicate_numbers_xor2(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut vis = 0i64;
    for x in nums {
        if vis >> x & 1 == 1 {
            result ^= x;
        } else {
            vis |= 1 << x;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 1, 3]), 1);
        assert_eq!(func(vec![1, 2, 3]), 0);
        assert_eq!(func(vec![1, 2, 2, 1]), 3);
    }
    test(duplicate_numbers_xor);
    test(duplicate_numbers_xor2);
}
