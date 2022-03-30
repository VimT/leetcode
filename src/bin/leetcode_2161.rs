//! 根据给定数字划分数组

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut small = Vec::with_capacity(nums.len());
    let mut big = vec![];
    let mut eq = 0;
    for num in nums {
        if num < pivot {
            small.push(num);
        } else if num == pivot {
            eq += 1;
        } else {
            big.push(num);
        }
    }
    for _ in 0..eq {
        small.push(pivot);
    }
    small.extend_from_slice(&big);
    small
}

fn main() {
    assert_eq!(pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10), vec![9, 5, 3, 10, 10, 12, 14]);
    assert_eq!(pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
}
