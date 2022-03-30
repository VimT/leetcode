//! 汉明距离总和

pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..32 {
        let mut one = 0;
        let mut zero = 0;
        for &num in &nums {
            if num >> i & 1 == 1 { one += 1 } else { zero += 1 }
        }
        result += one * zero;
    }
    result
}

fn main() {
    assert_eq!(total_hamming_distance(vec![4, 14, 2]), 6);
    assert_eq!(total_hamming_distance(vec![4, 14, 4]), 4);
}
