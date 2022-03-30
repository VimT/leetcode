//! 元素计数

pub fn count_elements(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut double_size = 2;
    for i in 1..len {
        if nums[i] == nums[0] {
            double_size += 1;
        } else {
            break;
        }
    }
    for i in (0..len - 1).rev() {
        if nums[i] == nums[len - 1] {
            double_size += 1;
        } else { break; }
    }
    if len >= double_size { (len - double_size) as i32 } else { 0 }
}

fn main() {
    assert_eq!(count_elements(vec![11, 7, 2, 15]), 2);
    assert_eq!(count_elements(vec![-3, 3, 3, 90]), 2);
}
