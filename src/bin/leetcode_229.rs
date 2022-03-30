//! 求众数 II

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut num1 = nums[0];
    let mut count1 = 0;
    let mut num2 = nums[0];
    let mut count2 = 0;
    let limit = (nums.len() / 3) as i32;
    let mut ans = vec![];
    for &i in &nums {
        if i == num1 {
            count1 += 1;
        } else if i == num2 {
            count2 += 1;
        } else {
            if count1 == 0 {
                num1 = i;
                count1 = 1;
            } else if count2 == 0 {
                num2 = i;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
    }
    count1 = 0;
    count2 = 0;
    for i in nums {
        if num1 == i { count1 += 1; }
        if num2 == i { count2 += 1; }
    }
    if count1 > limit { ans.push(num1); }
    if count2 > limit && num1 != num2 { ans.push(num2); }
    ans
}

fn main() {
    assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    assert_eq!(majority_element(vec![1]), vec![1]);
    assert_eq!(majority_element(vec![1, 2]), vec![1, 2]);
}
