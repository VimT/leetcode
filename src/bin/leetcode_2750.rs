//! 将数组划分成若干好子数组的方式

pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut result = 1;
    let mut last = -1;
    for (num, i) in nums.into_iter().zip(0..) {
        if num == 0 { continue; }
        if last != -1 {
            result = (result * (i - last)) % MOD;
        }
        last = i;
    }
    if last == -1 { 0 } else { result as i32 }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1, 0, 0, 1]), 3);
        assert_eq!(func(vec![0, 1, 0]), 1);
    }
    test(number_of_good_subarray_splits);
}
