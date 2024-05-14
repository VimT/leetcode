//! 判断一个数组是否可以变为有序

pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
    let len = nums.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if nums[j] > nums[j + 1] {
                if nums[j].count_ones() == nums[j + 1].count_ones() {
                    nums.swap(j, j + 1);
                } else {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![8, 4, 2, 30, 15]), true);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(func(vec![3, 16, 8, 4, 2]), false);
    }
    test(can_sort_array);
}
