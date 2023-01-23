//! 可被三整除的最大和

pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut m = vec![vec![]; 3];
    let mut sum = 0;
    for num in nums {
        sum += num;
        m[(num % 3) as usize].push(num);
    }
    for i in 0..3 {
        m[i].sort_unstable();
    }
    match sum % 3 {
        0 => sum,
        1 => {
            if m[1].len() > 0 { sum - m[1][0] } else { 0 }.max(if m[2].len() > 1 { sum - m[2][0] - m[2][1] } else { 0 })
        }
        2 => {
            if m[2].len() > 0 { sum - m[2][0] } else { 0 }.max(if m[1].len() > 1 { sum - m[1][0] - m[1][1] } else { 0 })
        }
        _ => {unreachable!()}
    }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 6, 5, 1, 8]), 18);
        assert_eq!(func(vec![4]), 0);
        assert_eq!(func(vec![1, 2, 3, 4, 4]), 12);
    }
    test(max_sum_div_three);
}
