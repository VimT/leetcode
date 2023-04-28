//! 找出可整除性得分最大的整数

pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    -divisors.into_iter().map(|div| {
        let mut cnt = 0;
        for &num in &nums {
            if num % div == 0 {
                cnt += 1;
            }
        }
        (cnt, -div)
    }).max().unwrap().1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, divisors: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 7, 9, 3, 9], vec![5, 2, 3]), 3);
        assert_eq!(func(vec![20, 14, 21, 10], vec![5, 7, 5]), 5);
        assert_eq!(func(vec![12], vec![10, 16]), 10);
    }
    test(max_div_score);
}
