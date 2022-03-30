//! 对奇偶下标分别排序

pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut odd = Vec::with_capacity(len);
    let mut even = Vec::with_capacity(len);
    for (i, num) in nums.into_iter().enumerate() {
        if i & 1 == 1 { &mut odd } else { &mut even }.push(num);
    }
    odd.sort_unstable();
    even.sort_unstable();
    odd.reverse();
    let mut result = Vec::with_capacity(len);
    for i in 0..len / 2 {
        result.push(even[i]);
        result.push(odd[i]);
    }
    if len & 1 == 1 { result.push(even[len / 2]); }
    result
}

fn main() {
    assert_eq!(sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
    assert_eq!(sort_even_odd(vec![2, 1]), vec![2, 1]);
    assert_eq!(sort_even_odd(vec![2, 1, 3]), vec![2, 1, 3]);
}
