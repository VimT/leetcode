//! 找出数组的串联值

pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut result = 0;
    while i <= j {
        if i == j  {
            result += nums[i] as i64;
        } else {
            let num = nums[i].to_string() + &nums[j].to_string();
            result += num.parse::<i64>().unwrap();
        }
        i += 1;
        j -= 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![7, 52, 2, 4]), 596);
        assert_eq!(func(vec![5, 14, 13, 8, 12]), 673);
    }
    test(find_the_array_conc_val);
}
