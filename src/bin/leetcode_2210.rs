//! 统计数组中峰和谷的数量

pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = nums.len();
    let mut i = 1;
    while i < len {
        let left = nums[i - 1];
        let num = nums[i];
        while i < len && nums[i] == num {
            i += 1;
        }
        if i == len { break; }
        let right = nums[i];
        if (left > num && right > num) || (left < num && right < num) {
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
    assert_eq!(count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
}
