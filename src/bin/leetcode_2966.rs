//! 划分数组并满足最大差限制

pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = Vec::with_capacity(len/3);
    for i in 0..len / 3 {
        result.push(nums[i * 3..i * 3 + 3].to_vec());
        if result[i][2] - result[i][0] > k {
            return vec![];
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2), vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]);
        assert_eq!(func(vec![1, 3, 3, 2, 7, 3], 3).is_empty(), true);
    }
    test(divide_array);
}
