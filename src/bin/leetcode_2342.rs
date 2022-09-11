//! 数位和相等数对的最大和

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut bit = vec![vec![]; 81];
    for num in nums {
        let mut i = num;
        let mut sum = 0;
        while i > 0 {
            sum += i % 10;
            i /= 10;
        }
        bit[sum as usize].push(num);
    }
    let mut result = -1;
    for mut arr in bit {
        if arr.len() > 1 {
            arr.sort_unstable();
            result = result.max(arr[arr.len() - 1] + arr[arr.len() - 2]);
        }
    }
    result
}

fn main() {
    assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
    assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
}