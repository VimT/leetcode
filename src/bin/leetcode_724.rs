//! find-pivot-index

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().sum::<i32>();
    let mut cur = 0;
    for i in 0..nums.len() {
        if sum - nums[i] == cur * 2 {
            return i as i32;
        }
        cur += nums[i];
    }
    return -1;
}

fn main() {
    assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(pivot_index(vec![0, 0, 0, 0, 1]), 4);
}
