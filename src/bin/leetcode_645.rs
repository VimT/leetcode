//! 错误的集合

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut vis = vec![false; len + 1];
    let mut dup = 0;
    for num in nums {
        if vis[num as usize] {
            dup = num;
        }
        vis[num as usize] = true;
    }
    for i in 1..=len {
        if !vis[i] {
            return vec![dup, i as i32];
        }
    }
    vec![]
}

fn main() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
}
