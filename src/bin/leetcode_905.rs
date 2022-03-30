//! 按奇偶排序数组

pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut result = vec![0; len];
    let mut i = 0;
    let mut j = len - 1;
    for num in nums {
        if num & 1 == 1 {
            result[j] = num;
            j -= 1;
        } else {
            result[i] = num;
            i += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(sort_array_by_parity(vec![3, 1, 2, 4]), vec![2, 4, 1, 3]);
    assert_eq!(sort_array_by_parity(vec![0]), vec![0]);
}
