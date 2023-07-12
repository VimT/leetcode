//! 最大子序列交替和

pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut a = 0;
    let mut b = 0;
    for num in nums {
        let na = a.max(b + num as i64);
        let nb = b.max(a - num as i64);
        a = na;
        b = nb;
    }
    a.max(b)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![4, 2, 5, 3]), 7);
        assert_eq!(func(vec![5, 6, 7, 8]), 8);
        assert_eq!(func(vec![6, 2, 1, 2, 4, 5]), 10);
    }
    test(max_alternating_sum);
}
