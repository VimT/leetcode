//! 对数组执行操作使平方和最大

/// 操作等价于：把一个数的 0 和另一个数的同一个比特位上的 1 交换。
pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut cnt = [0; 32];
    for num in nums {
        for i in 0..32 {
            if num >> i & 1 == 1 {
                cnt[i] += 1;
            }
        }
    }
    let mut result = 0;
    for _ in 0..k {
        let mut num = 0i64;
        for i in 0..32 {
            if cnt[i] > 0 {
                num |= 1 << i;
                cnt[i] -= 1;
            }
        }
        result = (result + num * num) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 6, 5, 8], 2), 261);
        assert_eq!(func(vec![4, 5, 4, 7], 3), 90);
    }
    test(max_sum);
}
