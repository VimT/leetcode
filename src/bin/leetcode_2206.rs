//! 将数组划分成相等数对

pub fn divide_array(nums: Vec<i32>) -> bool {
    let mut cnt = vec![0; 501];
    for num in nums {
        cnt[num as usize] += 1;
    }
    for i in 1..=500 {
        if cnt[i] & 1 == 1 {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(divide_array(vec![3, 2, 3, 2, 2, 2]), true);
    assert_eq!(divide_array(vec![1, 2, 3, 4]), false);
}
