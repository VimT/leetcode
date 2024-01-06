//! 统计移除递增子数组的数目 I

pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        for j in i..len {
            let mut last = i32::MIN;
            let mut ok = true;
            for num in nums[0..i].iter().chain(&nums[j + 1..]) {
                if *num > last {
                    last = *num;
                } else {
                    ok = false;
                    break;
                }
            }
            if ok { result += 1; }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4]), 10);
        assert_eq!(func(vec![6, 5, 7, 8]), 7);
        assert_eq!(func(vec![8, 7, 6, 6]), 3);
    }
    test(incremovable_subarray_count);
}
