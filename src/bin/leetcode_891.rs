//! 子序列宽度之和

pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = nums.len();
    nums.sort_unstable();
    let mut pow2 = vec![0; len];
    pow2[0] = 1;
    for i in 1..len {
        pow2[i] = pow2[i - 1] * 2 % MOD;
    }
    let mut result = 0;
    for i in 0..len {
        result = (result + (pow2[i] - pow2[len - 1 - i]) * nums[i] as i64) % MOD;
    }
    result as i32
}

fn main() {
    assert_eq!(sum_subseq_widths(vec![2, 1, 3]), 6);
    assert_eq!(sum_subseq_widths(vec![2]), 0);
}
