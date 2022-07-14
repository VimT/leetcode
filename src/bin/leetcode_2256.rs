//! 最小平均差

pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
    let sum: i64 = nums.iter().map(|x| *x as i64).sum();
    let mut cursum = 0;
    let len = nums.len();
    let mut min_avg = i64::MAX;
    let mut result = 0;
    for i in 0..len {
        cursum += nums[i] as i64;
        let avg = (cursum / (i as i64 + 1) - if i == len - 1 { 0 } else { (sum - cursum) / (len - i - 1) as i64 }).abs();
        if avg < min_avg {
            min_avg = avg;
            result = i as i32;
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_average_difference(vec![2, 5, 3, 9, 5, 3]), 3);
    assert_eq!(minimum_average_difference(vec![0]), 0);
}
