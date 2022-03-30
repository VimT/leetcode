//! 分组得分最高的所有下标

pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut zero = vec![0; len + 1];
    let mut one = vec![0; len + 1];
    let mut cur = 0;
    for i in 0..len {
        if nums[i] == 0 { cur += 1; }
        zero[i + 1] = cur;
    }
    cur = 0;
    for i in (0..len).rev() {
        if nums[i] == 1 { cur += 1; }
        one[i] = cur;
    }
    let mut max = 0;
    let mut result = vec![];
    for i in 0..=len {
        if zero[i] + one[i] > max {
            max = zero[i] + one[i];
            result = vec![i as i32];
        } else if zero[i] + one[i] == max {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(max_score_indices(vec![0, 0, 1, 0]), vec![2, 4]);
    assert_eq!(max_score_indices(vec![0, 0, 0]), vec![3]);
    assert_eq!(max_score_indices(vec![1, 1]), vec![0]);
}
