//! 数组中的 k-diff 数对

pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    let mut result = 0;
    let mut j = 1;
    for i in 0..len {
        if i > 0 && nums[i] == nums[i - 1] { continue; }
        while (j < len && nums[j] < nums[i] + k) || (j == i) {
            j += 1;
        }
        if j == len { break; }
        if nums[j] == nums[i] + k { result += 1; }
    }
    result
}

fn main() {
    assert_eq!(find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    assert_eq!(find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    assert_eq!(find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
}
