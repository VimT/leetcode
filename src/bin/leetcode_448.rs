//! 找到所有数组中消失的数字

pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    for i in 0..len {
        let idx = (nums[i] as usize - 1) % len;
        nums[idx] += len as i32;
    }
    nums.into_iter().enumerate().filter(|(_, x)| *x <= len as i32).map(|(idx, _)| idx as i32 + 1).collect()
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![5, 6]);
        assert_eq!(func(vec![1, 1]), vec![2]);
    }
    test(find_disappeared_numbers);
}
