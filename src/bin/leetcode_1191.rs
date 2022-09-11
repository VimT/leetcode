//! K 次串联后最大子数组之和

pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = arr.len();
    let mut cur_sum = 0;
    let mut max = 0;
    let mut sum = 0;
    for i in 0..len {
        cur_sum = (cur_sum + arr[i]).max(arr[i]).max(0);
        max = max.max(cur_sum);
        sum += arr[i];
    }
    if k == 1 {
        return max;
    }
    if max == 0 {
        return 0;
    }
    for i in 0..len {
        cur_sum = (cur_sum + arr[i]).max(arr[i]).max(0);
        max = max.max(cur_sum);
    }
    if sum < 0 {
        return max;
    }
    let sumk = (sum as i64) * (k as i64 - 2) + max as i64;
    (sumk % MOD) as i32
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, -5, 1, 0, -2, -2, 2], 2), 4);
        assert_eq!(func(vec![1, 2], 3), 9);
        assert_eq!(func(vec![1, -2, 1], 5), 2);
        assert_eq!(func(vec![-1, -2], 7), 0);
    }
    test(k_concatenation_max_sum);
}
