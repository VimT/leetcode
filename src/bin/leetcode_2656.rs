//! K 个元素的最大和

pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let max = *nums.iter().max().unwrap();
    (max * 2 + k - 1) * k / 2
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5], 3), 18);
        assert_eq!(func(vec![5, 5, 5], 2), 11);
    }
    test(maximize_sum);
}
