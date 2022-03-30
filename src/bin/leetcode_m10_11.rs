//! 面试题 10.11. 峰与谷

// O(n)
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        if i % 2 == 0 {
            if nums[i] < nums[i - 1] {
                nums.swap(i, i - 1);
            }
        } else {
            if nums[i] > nums[i - 1] {
                nums.swap(i, i - 1);
            }
        }
    }
}

fn main() {
    let mut num = vec![5, 3, 1, 2, 3];
    wiggle_sort(&mut num);
    assert_eq!(num, vec![5, 1, 3, 2, 3]);
}
