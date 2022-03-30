//! 最大间距

/// Radix sort
pub fn maximum_gap_radix_sort(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 2 { return 0; }
    let mut redix = vec![vec![]; 10];
    let mut max = *nums.iter().max().unwrap();
    let mut exp = 1;
    while max > 0 {
        for &i in &nums {
            redix[i as usize / exp % 10].push(i);
        }
        let mut idx = 0;
        for i in 0..10 {
            for &n in &redix[i] {
                nums[idx] = n;
                idx += 1;
            }
            redix[i].clear();
        }
        max /= 10;
        exp *= 10;
    }


    let mut ans = 0;
    for i in 1..nums.len() {
        ans = ans.max(nums[i] - nums[i - 1]);
    }
    ans
}

/// bucket sort
/// set bucket num = len - 1, ensure max gap will not product in bucket
/// then ans = (next bucket).min - (this bucket).max
pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 { return 0; }
    let len = nums.len();
    let mut min = nums[0];
    let mut max = nums[0];
    for &i in &nums[1..len] {
        min = min.min(i);
        max = max.max(i);
    }
    if max - min == 0 { return 0; }
    let interval = ((max - min) as f64 / (len - 1) as f64).ceil() as i32;
    let mut bucket_min = vec![i32::MAX; len - 1];
    let mut bucket_max = vec![-1; len - 1];

    for &n in &nums {
        let idx = ((n - min) / interval) as usize;
        if n == min || n == max { continue; }
        bucket_min[idx] = n.min(bucket_min[idx]);
        bucket_max[idx] = n.max(bucket_max[idx]);
    }

    let mut ans = 0;
    let mut previous_max = min;
    for i in 0..len - 1 {
        if bucket_max[i] == -1 { continue; }
        ans = ans.max(bucket_min[i] - previous_max);
        previous_max = bucket_max[i];
    }
    ans.max(max - previous_max)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 6, 9, 1]), 3);
        assert_eq!(func(vec![10]), 0);
    }
    test(maximum_gap);
    test(maximum_gap_radix_sort);
}
