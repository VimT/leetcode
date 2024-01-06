//! 滑动子数组的美丽值

/// 如果数据范围更大：名次树
pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut cnt = [0; 101];
    let k = k as usize;
    for i in 0..k - 1 {
        cnt[(nums[i] + 50) as usize] += 1;
    }
    let mut result = vec![0; nums.len() - k + 1];
    for ((&a, &b), i) in nums[k - 1..].iter().zip(&nums).zip(0..) {
        cnt[(a + 50) as usize] += 1;
        let mut sum = 0;
        for j in 0..50 {
            sum += cnt[j];
            if sum >= x {
                result[i] = j as i32 - 50;
                break;
            }
        }
        cnt[(b + 50) as usize] -= 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, -1, -3, -2, 3], 3, 2), vec![-1, -2, -2]);
        assert_eq!(func(vec![-1, -2, -3, -4, -5], 2, 2), vec![-1, -2, -3, -4]);
        assert_eq!(func(vec![-3, 1, 2, -3, 0, -3], 2, 1), vec![-3, 0, -3, -3, -3]);
    }
    test(get_subarray_beauty);
}
