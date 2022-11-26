//! 数组中不等三元组的数目

pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            if nums[i] != nums[j] {
                for k in j + 1..len {
                    if nums[j] != nums[k] && nums[i] != nums[k] {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 4, 2, 4, 3]), 3);
        assert_eq!(func(vec![1, 1, 1, 1, 1]), 0);
    }
    test(unequal_triplets);
}
