//! 数组中重复的数据

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    for i in 0..len {
        let idx = (nums[i] - 1) as usize % len;
        nums[idx] += len as i32;
    }
    let mut result = vec![];
    for i in 0..len {
        if nums[i] > 2 * len as i32 && nums[i] <= 3 * len as i32 {
            result.push(1 + i as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
    assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
    assert_eq!(find_duplicates(vec![1, 3, 3]), vec![3]);
    assert_eq!(find_duplicates(vec![1]), vec![]);
}
