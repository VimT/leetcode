//! 统计最大元素出现至少 K 次的子数组


pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mut result = 0;
    let k = k as usize;
    let mut max_num_pos = vec![];
    let max_num = *nums.iter().max().unwrap();
    for (i, num) in nums.into_iter().enumerate() {
        if num == max_num {
            max_num_pos.push(i);
        }
        if max_num_pos.len() >= k {
            result += max_num_pos[max_num_pos.len() - k] as i64 + 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![28, 5, 58, 91, 24, 91, 53, 9, 48, 85, 16, 70, 91, 91, 47, 91, 61, 4, 54, 61, 49], 1), 187);
        assert_eq!(func(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(func(vec![1, 4, 2, 1], 3), 0);
    }
    test(count_subarrays);
}
