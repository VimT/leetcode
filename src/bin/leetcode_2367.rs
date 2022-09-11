//! 算术三元组的数目

pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        for j in 0..len {
            for k in 0..len {
                if nums[k] - nums[j] == diff && nums[j] - nums[i] == diff {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, diff: i32) -> i32) {
        assert_eq!(func(vec![0, 1, 4, 6, 7, 10], 3), 2);
        assert_eq!(func(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
    test(arithmetic_triplets);
}
