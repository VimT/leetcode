//! 非递减数列

pub fn check_possibility(mut nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut cnt = 0;
    for i in 0..len - 1 {
        if nums[i] > nums[i + 1] {
            cnt += 1;
            if cnt > 1 {
                return false;
            }
            if i > 0 && nums[i - 1] > nums[i + 1] {
                nums[i + 1] = nums[i];
            }
        }
    }
    true
}

fn main() {
    assert_eq!(check_possibility(vec![4, 2, 3]), true);
    assert_eq!(check_possibility(vec![4, 2, 1]), false);
}
