//! 找出最长等值子数组

pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut pos = vec![vec![]; len + 1];
    for (i, num) in nums.into_iter().enumerate() {
        pos[num as usize].push(i);
    }
    let mut result = 0;
    let k = k as usize;
    for list in pos {
        let mut l = 0;
        for r in 0..list.len() {
            while l < r && l + list[r] - r - list[l] > k {
                l += 1;
            }
            result = result.max(r - l + 1);
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 3, 2, 3, 1, 3], 3), 3);
        assert_eq!(func(vec![1, 1, 2, 2, 1, 1], 2), 4);
    }
    test(longest_equal_subarray);
}
