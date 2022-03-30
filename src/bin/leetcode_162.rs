//! 寻找峰值

pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    fn inner(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if start == end {
            return start as i32;
        }
        let mid = (end + start) / 2;
        return if nums[mid] > nums[mid + 1] {
            inner(nums, start, mid)
        } else {
            inner(nums, mid + 1, end)
        };
    }
    return inner(&nums, 0, nums.len() - 1);
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 1]), 2);
        assert_eq!(func(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
    test(find_peak_element);
}
