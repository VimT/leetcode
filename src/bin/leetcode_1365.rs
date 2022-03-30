//! 有多少小于当前数字的数字

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut counter = vec![0; 101];

    for &i in &nums {
        counter[i as usize] += 1;
    }
    for i in 1..=100 {
        counter[i] += counter[i - 1];
    }
    return nums.into_iter().map(|x| if x > 0 { counter[x as usize - 1] } else { 0 }).collect();
}

fn main() {
    assert_eq!(smaller_numbers_than_current(vec![8, 1, 2, 2, 3]), vec![4, 0, 1, 1, 3]);
    assert_eq!(smaller_numbers_than_current(vec![6, 5, 4, 8]), vec![2, 1, 0, 3]);
    assert_eq!(smaller_numbers_than_current(vec![7, 7, 7, 7]), vec![0, 0, 0, 0]);
}
