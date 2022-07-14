//! 缺失的区间

pub fn find_missing_ranges(mut nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
    nums.insert(0, lower - 1);
    nums.push(upper + 1);
    let len = nums.len();
    let mut result = vec![];
    for i in 1..len {
        if nums[i] - nums[i - 1] == 2 {
            result.push((nums[i - 1] + 1).to_string());
        } else if nums[i] - nums[i - 1] > 2 {
            result.push(format!("{}->{}", nums[i - 1] + 1, nums[i] - 1));
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String>) {
        assert_eq!(func(vec![], 1, 1), vec!["1"]);
        assert_eq!(func(vec![-1000000000, -9999, 0, 1, 2, 10, 100, 1000, 999999999, 1000000000], -1000000000, 1000000000), vec!["-999999999->-10000", "-9998->-1", "3->9", "11->99", "101->999", "1001->999999998"]);
        assert_eq!(func(vec![0, 1, 3, 50, 75], 0, 99), vec!["2", "4->49", "51->74", "76->99"]);
        assert_eq!(func(vec![-1], -1, -1).is_empty(), true);
    }
    test(find_missing_ranges);
}
