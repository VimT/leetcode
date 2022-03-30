//! 分割数组

pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut right_min = vec![i32::MAX; len];
    let mut cur = nums[len - 1];
    for i in (0..len).rev() {
        cur = cur.min(nums[i]);
        right_min[i] = cur;
    }
    cur = nums[0];
    for i in 0..len - 1 {
        if right_min[i + 1] >= cur {
            return 1 + i as i32;
        }
        cur = cur.max(nums[i]);
    }
    0
}

fn main() {
    assert_eq!(partition_disjoint(vec![90, 47, 69, 10, 43, 92, 31, 73, 61, 97]), 9);
    assert_eq!(partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    assert_eq!(partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
}
