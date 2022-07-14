//! 摆动排序

pub fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort_unstable();
    for i in (2..nums.len()).step_by(2) {
        nums.swap(i, i - 1);
    }
}

fn main() {
    fn test(func: fn(nums: &mut Vec<i32>)) {
        let help = |mut nums: Vec<i32>| {
            func(&mut nums);
            for i in (1..nums.len()).step_by(2) {
                if nums[i] < nums[i - 1] || (i + 1 < nums.len() && nums[i] < nums[i + 1]) {
                    return false;
                }
            }
            true
        };
        assert_eq!(help(vec![3, 5, 2, 1, 6, 4]), true);
        assert_eq!(help(vec![6, 6, 5, 6, 3, 8]), true);
    }
    test(wiggle_sort);
}
