//! 数组中的最大数对和

pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut mx = [0; 10];
    let mut result = -1;
    for x in nums {
        let mut s = 0;
        let mut num = x;
        while num > 0 {
            s = s.max(num % 10);
            num /= 10;
        }
        if mx[s as usize] != 0 {
            result = result.max(x + mx[s as usize]);
        }
        mx[s as usize] = mx[s as usize].max(x);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![51, 71, 17, 24, 42]), 88);
        assert_eq!(func(vec![1, 2, 3, 4]), -1);
    }
    test(max_sum);
}
