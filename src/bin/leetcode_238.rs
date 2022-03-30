//! 除自身以外数组的乘机

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut left = vec![1; len];
    let mut right = vec![1; len];
    for i in 1..len {
        left[i] = nums[i - 1] * left[i - 1];
        right[len - i - 1] *= nums[len - i] * right[len - i]
    }

    left.into_iter().zip(right).map(|(x, y)| x * y).collect()
}

fn main() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6])
}
