//! 乘积最大子数组


pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];

    for i in 0..nums.len() {
        for j in i..nums.len() {
            let product = nums[i..=j].iter().fold(1, |a, b| a * b);
            if product > max {
                max = product;
            };
        }
    }
    max
}

pub fn max_product2(nums: Vec<i32>) -> i32 {
    use std::mem::swap;

    let mut maxv = 1;
    let mut minv = 1;

    let mut ret = nums[0];
    for i in nums {
        if i < 0 { swap(&mut maxv, &mut minv); }
        maxv = i.max(maxv * i);
        minv = i.min(minv * i);
        ret = ret.max(maxv);
    }
    ret
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, -2, 4]), 6);
        assert_eq!(func(vec![-2, 0, -1]), 0);
    }
    test(max_product);
    test(max_product2);
}
