//! 三角形的最大周长

pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    for i in (2..len).rev() {
        if nums[i - 2] + nums[i - 1] > nums[i] {
            return nums[i - 1] + nums[i - 2] + nums[i];
        }
    }
    0
}

fn main() {
    assert_eq!(largest_perimeter(vec![2, 1, 2]), 5);
    assert_eq!(largest_perimeter(vec![1, 2, 1]), 0);
}
