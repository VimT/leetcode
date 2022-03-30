//! 组合总和 II

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn inner(nums: &Vec<i32>, idx: usize, current: &mut Vec<i32>, left: i32, ans: &mut Vec<Vec<i32>>) {
        if left == 0 { ans.push(current.clone()); }
        if idx > nums.len() { return; }
        for i in idx..nums.len() {
            let num = nums[i];
            if num > left { break; }
            if i > idx && nums[i] == nums[i - 1] { continue; }
            current.push(num);
            inner(nums, i + 1, current, left - num, ans);
            current.pop();
        }
    }

    candidates.sort_unstable();
    let mut ans = vec![];
    inner(&candidates, 0, &mut vec![], target, &mut ans);
    ans
}

fn main() {
    assert_eq!(combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8), [
        vec![1, 1, 6],
        vec![1, 2, 5],
        vec![1, 7],
        vec![2, 6]
    ]);
    assert_eq!(combination_sum2(vec![2, 5, 2, 1, 2], 5), [
        vec![1, 2, 2],
        vec![5]
    ]);
}
