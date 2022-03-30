//! 乘积小于K的子数组

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 { return 0; }
    let mut cur = 1;
    let mut left = 0;
    let mut result = 0;
    for (right, val) in nums.iter().enumerate() {
        cur *= val;
        while cur >= k {
            cur /= nums[left];
            left += 1;
        }
        result += (right + 1 - left) as i32;
    }
    result
}

fn main() {
    assert_eq!(num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
    assert_eq!(num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
}