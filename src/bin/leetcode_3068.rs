//! 最大节点价值之和

pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _: Vec<Vec<i32>>) -> i64 {
    let mut xor_diff: Vec<i32> = nums.iter().map(|&x| (x ^ k) - x).collect();
    let mut result = nums.iter().map(|&x| x as i64).sum::<i64>();
    xor_diff.sort_unstable();
    while xor_diff.len() >= 2 {
        let a = xor_diff.pop().unwrap();
        let b = xor_diff.pop().unwrap();
        if a + b > 0 {
            result += a as i64 + b as i64;
        } else { break; }
    }
    result
}

pub fn maximum_value_sum2(nums: Vec<i32>, k: i32, _: Vec<Vec<i32>>) -> i64 {
    let mut f0 = 0;
    let mut f1 = i64::MIN;
    let k = k as i64;
    for num in nums {
        let num = num as i64;
        let f0_new = (f0 + num).max(f1 + (num ^ k));
        let f1_new = (f1 + num).max(f0 + (num ^ k));
        f0 = f0_new;
        f1 = f1_new;
    }
    f0
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]]), 6);
        assert_eq!(func(vec![2, 3], 7, vec![vec![0, 1]]), 9);
        assert_eq!(func(vec![7, 7, 7, 7, 7, 7], 3, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]), 42);
    }
    test(maximum_value_sum);
    test(maximum_value_sum2);
}
