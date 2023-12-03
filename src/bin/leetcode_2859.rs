//! 计算 K 置位下标对应元素的和

pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        if i.count_ones() == k as u32 {
            result += nums[i];
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![5, 10, 1, 5, 2], 1), 13);
        assert_eq!(func(vec![4, 3, 2, 1], 2), 1);
    }
    test(sum_indices_with_k_set_bits);
}
