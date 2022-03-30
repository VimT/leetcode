//! 有序数组的平方

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mid = nums.binary_search(&0).unwrap_or_else(|x| x);
    let mut i = mid;
    let mut j = mid;
    let len = nums.len();
    let mut result = Vec::with_capacity(len);
    while i < len && j > 0 {
        if nums[i] > -nums[j - 1] {
            result.push(nums[j - 1] * nums[j - 1]);
            j -= 1;
        } else {
            result.push(nums[i] * nums[i]);
            i += 1;
        }
    }
    for k in (0..j).rev() {
        result.push(nums[k] * nums[k]);
    }
    for k in i..len {
        result.push(nums[k] * nums[k]);
    }
    result
}

fn main() {
    assert_eq!(sorted_squares(vec![-5, -3, -2, -1]), vec![1, 4, 9, 25]);
    assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    assert_eq!(sorted_squares(vec![-7, -3, 2, 3, 11]), vec![4, 9, 9, 49, 121]);
}
