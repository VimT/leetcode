//! 按既定顺序创建目标数组

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for (num, idx) in nums.into_iter().zip(index) {
        result.insert(idx as usize, num);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]), vec![0, 4, 1, 3, 2]);
        assert_eq!(func(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]), vec![0, 1, 2, 3, 4]);
        assert_eq!(func(vec![1], vec![0]), vec![1]);
    }
    test(create_target_array);
}
