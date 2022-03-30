//! 最长湍流子数组

pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut dec = vec![1; len];
    let mut inc = vec![1; len];
    let mut result = 1;
    for i in 1..len {
        if arr[i] > arr[i - 1] {
            inc[i] = dec[i - 1] + 1;
            result = result.max(inc[i]);
        } else if arr[i] < arr[i - 1] {
            dec[i] = inc[i - 1] + 1;
            result = result.max(dec[i]);
        }
    }
    result
}

fn main() {
    assert_eq!(max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    assert_eq!(max_turbulence_size(vec![4, 8, 12, 16]), 2);
    assert_eq!(max_turbulence_size(vec![100]), 1);
}
