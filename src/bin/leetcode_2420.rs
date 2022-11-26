//! 找到所有好下标

pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    let mut left_down = vec![1; len];
    let mut right_up = vec![1; len];
    let mut cur_down = 1;
    for i in 1..len-1 {
        if nums[i] <= nums[i - 1] {
            cur_down += 1;
            left_down[i+1] = cur_down;
        } else {
            cur_down = 1;
        }
    }
    let mut cur_up = 1;
    for i in (1..len - 1).rev() {
        if nums[i] <= nums[i + 1] {
            cur_up += 1;
            right_up[i-1] = cur_up;
        } else {
            cur_up = 1;
        }
    }
    let mut result = vec![];
    for i in k as usize..len - k as usize {
        if left_down[i] >= k && right_up[i] >= k {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(good_indices(vec![878724, 201541, 179099, 98437, 35765, 327555, 475851, 598885, 849470, 943442], 4), vec![4, 5]);
    assert_eq!(good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2), vec![2, 3]);
    assert_eq!(good_indices(vec![2, 1, 1, 2], 2), vec![]);
}
