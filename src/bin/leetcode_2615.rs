//! 等值距离和

use std::collections::HashMap;

pub fn distance(nums: Vec<i32>) -> Vec<i64> {
    let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
    let len = nums.len();
    for i in 0..len {
        m.entry(nums[i]).or_default().push(i);
    }
    let mut result = vec![0; len];
    for (_, idx) in m {
        let len = idx.len();
        let sum: usize = idx.iter().sum();
        let mut cursum = 0;
        for (i, pos) in idx.into_iter().enumerate() {
            result[pos] = ((pos * i - cursum) + (sum - cursum - pos * (len - i))) as i64;
            cursum += pos;
        }
    }
    result
}

/// 做法2：算出a[0]的距离和，a[i]的距离和可以从a[i-1]的距离和推导
pub fn distance2(nums: Vec<i32>) -> Vec<i64> {
    let mut m: HashMap<i32, Vec<i64>> = HashMap::new();
    let len = nums.len();
    for i in 0..len {
        m.entry(nums[i]).or_default().push(i as i64);
    }
    let mut result = vec![0; len];
    for (_, a) in m {
        let len = a.len() as i64;
        let mut sum: i64 = a.iter().map(|&x| (x - a[0]) as i64).sum();
        result[a[0] as usize] = sum;
        for (i, &pos) in a.iter().enumerate().skip(1) {
            sum += (2 * i as i64 - len) * (a[i] - a[i - 1]);
            result[pos as usize] = sum;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i64>) {
        assert_eq!(func(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
        assert_eq!(func(vec![0, 5, 3]), vec![0, 0, 0]);
    }
    test(distance);
    test(distance2);
}
