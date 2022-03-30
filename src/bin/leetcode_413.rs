//! 等差数列划分

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut result = 0;
    let len = nums.len();
    if len < 3 { return 0; }
    while i < len - 2 {
        let diff = nums[i + 1] - nums[i];
        let mut right = i + 2;
        while right < len && nums[right] - nums[right - 1] == diff { right += 1; }
        right -= 1;
        let len = right - i + 1;
        if len >= 3 {
            result += (len - 1) * (len - 2) / 2;
            i = right;
        } else {
            i += 1;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 4]), 0);
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3]), 1);
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
    assert_eq!(number_of_arithmetic_slices(vec![1]), 0);
}
