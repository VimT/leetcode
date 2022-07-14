//! 划分数组使最大差为 K

pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut i = 0;
    let mut result = 0;
    while i < len {
        let mut j = i + 1;
        while j < len && nums[j] - nums[i] <= k {
            j += 1;
        }
        result += 1;
        i = j;
    }
    result
}

fn main() {
    assert_eq!(partition_array(vec![3, 6, 1, 2, 5], 2), 2);
    assert_eq!(partition_array(vec![1, 2, 3], 1), 2);
    assert_eq!(partition_array(vec![2, 2, 4, 5], 0), 3);
}
