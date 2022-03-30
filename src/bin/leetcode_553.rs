//! 最优除法

pub fn optimal_division(nums: Vec<i32>) -> String {
    let len = nums.len();
    if len == 1 { return nums[0].to_string(); }
    if len == 2 { return format!("{}/{}", nums[0], nums[1]); }
    let nums: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();
    format!("{}/({})", nums[0], nums[1..].join("/"))
}

fn main() {
    assert_eq!(optimal_division(vec![1000, 100, 10, 2]), String::from("1000/(100/10/2)"));
    assert_eq!(optimal_division(vec![2, 3, 4]), String::from("2/(3/4)"));
    assert_eq!(optimal_division(vec![2]), String::from("2"));
    assert_eq!(optimal_division(vec![2, 3]), String::from("2/3"));
}
