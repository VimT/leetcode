//! 所有数对中数位不同之和

pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let m = nums[0].to_string().len();
    let mut cnt = vec![[0; 10]; m];
    let mut result = (m * n * (n - 1) / 2) as i64;
    for num in nums {
        let mut num = num as usize;
        for i in 0..m {
            result -= cnt[i][num % 10];
            cnt[i][num % 10] += 1;
            num /= 10;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![13, 23, 12]), 4);
        assert_eq!(func(vec![10, 10, 10, 10]), 0);
    }
    test(sum_digit_differences);
}
