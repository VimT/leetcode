//! 最大连续1的个数 III

pub fn longest_ones(mut a: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut ans = 0;
    let k = k as usize;
    let len = a.len();
    let mut cnt = 0;
    for right in 0..len {
        if a[right] == 0 {
            while cnt >= k && left <= right {
                if a[left] == 2 {
                    cnt -= 1;
                }
                left += 1;
            }
            a[right] += 2;
            cnt += 1;
        }
        ans = ans.max(right + 1 - left);
    }
    ans as i32
}

// lsum = sum(1-a[0..left]), rsum = sum(1-a[0..right])   mean zero count
pub fn longest_ones_prefix_sum(a: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut ans = 0;
    let len = a.len();
    let mut lsum = 0;
    let mut rsum = 0;
    for right in 0..len {
        rsum += 1 - a[right];
        while rsum - lsum > k {
            lsum += 1 - a[left];
            left += 1;
        }
        ans = ans.max(right + 1 - left);
    }
    ans as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(func(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3), 10);
    }
    test(longest_ones);
    test(longest_ones_prefix_sum);
}
