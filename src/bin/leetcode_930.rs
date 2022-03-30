//! 和相同的二元子数组

pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let len = nums.len();
    let mut cnt = vec![0; len + 1];
    let mut cur = 0;
    let mut result = 0;
    for num in nums {
        cnt[cur as usize] += 1;
        cur += num;
        if cur >= goal {
            result += cnt[(cur - goal) as usize];
        }
    }
    result
}

fn main() {
    assert_eq!(num_subarrays_with_sum(vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0], 0), 27);
    assert_eq!(num_subarrays_with_sum(vec![1, 1, 1, 1, 1], 5), 1);
    assert_eq!(num_subarrays_with_sum(vec![0, 1, 1, 1, 1], 3), 3);
    assert_eq!(num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    assert_eq!(num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
}
