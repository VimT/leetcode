//! 数组中两元素的最大乘积

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut m1 = 0;
    let mut m2 = 0;
    for num in nums {
        if num > m1 {
            m2 = m1;
            m1 = num;
        } else if num > m2 {
            m2 = num;
        }
    }
    (m1 - 1) * (m2 - 1)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 4, 5, 2]), 12);
        assert_eq!(func(vec![1, 5, 4, 5]), 16);
        assert_eq!(func(vec![3, 7]), 12);
    }
    test(max_product);
}
