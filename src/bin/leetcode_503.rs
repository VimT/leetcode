//! 下一个更大元素 II

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len == 0 { return vec![]; }
    let mut q = std::collections::VecDeque::with_capacity(len);
    let mut ans = vec![-1; len];
    for i in 0..2 * len - 1 {
        while !q.is_empty() && nums[i % len] > nums[*q.back().unwrap() % len] {
            let idx = q.pop_back().unwrap();
            ans[idx % len] = nums[i % len];
        }
        q.push_back(i);
    }
    ans
}


fn main() {
    assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
    assert_eq!(next_greater_elements(vec![1, 2, 3, 4, 3]), vec![2, 3, 4, -1, 4]);
}
