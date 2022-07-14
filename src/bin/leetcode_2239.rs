//! 找到最接近 0 的数字

pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    for num in nums {
        if num.abs() < result.abs() || (num.abs() == result.abs() && num > 0) {
            result = num;
        }
    }
    result
}

fn main() {
    assert_eq!(find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
    assert_eq!(find_closest_number(vec![2, -1, 1]), 1);
}
