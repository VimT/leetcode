//! 大小为 K 且平均值大于等于阈值的子数组数目

pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let mut result = 0;
    let mut cur_sum = 0;
    for i in 0..k as usize {
        cur_sum += arr[i];
    }
    if cur_sum / k >= threshold {
        result += 1;
    }
    for i in k as usize..arr.len() {
        cur_sum += arr[i];
        cur_sum -= arr[i - k as usize];
        if cur_sum / k >= threshold {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32, threshold: i32) -> i32) {
        assert_eq!(func(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
        assert_eq!(func(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5), 6);
    }
    test(num_of_subarrays);
}
