//! 操作后的最大异或和

pub fn maximum_xor(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for num in nums {
        result |= num;
    }
    result
}

fn main() {
    assert_eq!(maximum_xor(vec![3, 2, 4, 6]), 7);
    assert_eq!(maximum_xor(vec![1, 2, 3, 9, 2]), 11);
}
