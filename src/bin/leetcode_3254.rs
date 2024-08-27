//! 长度为 K 的子数组的能量值 I

pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    let mut i = 0;
    let k = k as usize;
    let mut result = vec![-1; len + 1 - k];
    while i < len {
        let s = i;
        let mut cur = nums[i];
        while i < len && nums[i] == cur {
            if i + 1 >= k && i - s + 1 >= k {
                result[i + 1 - k] = cur;
            }
            cur += 1;
            i += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 2, 3, 4, 3, 2, 5], 3), vec![3, 4, -1, -1, -1]);
        assert_eq!(func(vec![2, 2, 2, 2, 2], 4), vec![-1, -1]);
        assert_eq!(func(vec![3, 2, 3, 2, 3, 2], 2), vec![-1, 3, -1, 3, -1]);
    }
    test(results_array);
}
