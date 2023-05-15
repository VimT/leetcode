//! 英雄的力量

pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut result = 0;
    let mut pps = 0;
    const MOD: i64 = 1e9 as i64 + 7;
    for num in nums {
        let num = num as i64;
        result += (num * num % MOD) * (pps + num) % MOD;
        result %= MOD;
        pps = (pps + pps + num) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1, 4]), 141);
        assert_eq!(func(vec![1, 1, 1]), 7);
    }
    test(sum_of_power);
}
