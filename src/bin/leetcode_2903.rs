//! 找出满足差值条件的下标 I

pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let len = nums.len();
    for i in 0..len {
        for j in i + index_difference as usize..len {
            if (nums[j] - nums[i]).abs() >= value_difference {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![-1, -1]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32>) {
        assert_eq!(func(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
        assert_eq!(func(vec![2, 1], 0, 0), vec![0, 0]);
        assert_eq!(func(vec![1, 2, 3], 2, 4), vec![-1, -1]);
    }
    test(find_indices);
}
