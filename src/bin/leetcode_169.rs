//! 多数元素

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 1;
    let mut last = nums[0];

    for i in 1..nums.len() {
        let num = nums[i];
        if count == 0 {
            last = num;
        }
        if num == last {
            count += 1;
        } else {
            count -= 1;
        }
    }
    return last;
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 3]), 3);
        assert_eq!(func(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
    test(majority_element);
}
