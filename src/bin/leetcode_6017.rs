//! 向数组中追加 K 个整数

pub fn minimal_k_sum(mut nums: Vec<i32>, mut k: i32) -> i64 {
    nums.push(0);
    nums.push(2e9 as i32);
    nums.sort_unstable();
    let mut result = 0;
    for i in 1..nums.len() {
        let fill = nums[i] - nums[i - 1] - 1;
        if fill <= 0 { continue; }
        if fill >= k {
            result += (nums[i - 1] as i64 * 2 + 1 + k as i64) * k as i64 / 2;
            return result;
        }
        result += (nums[i - 1] + nums[i]) as i64 * fill as i64 / 2;
        k -= fill;
    }
    result
}

fn main() {
    assert_eq!(minimal_k_sum(vec![96, 44, 99, 25, 61, 84, 88, 18, 19, 33, 60, 86, 52, 19, 32, 47, 35, 50, 94, 17, 29, 98, 22, 21, 72, 100, 40, 84], 35), 794);
    assert_eq!(minimal_k_sum(vec![1, 4, 25, 10, 25], 2), 5);
    assert_eq!(minimal_k_sum(vec![5, 6], 6), 25);
}
