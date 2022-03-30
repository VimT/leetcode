//! 使数组互补的最少操作次数

/// 差分数组
/// 如果修改后两个数字的和是 A + B，我们使用的操作数是 0 (没有修改));
/// 否则的话，如果修改后两个数字和在 [1 + min(A, B), limit + max(A, B)] 的范围，我们使用的操作数是 1 (只需要修改 A 或者 B 就好);
/// 否则的话，如果修改后两个数字和在 [2, 2 * limit] 的范围，我们使用的操作数是 ``2`(两个数字都要修改));
pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
    let len = nums.len();
    let mut diff = vec![0; 2 * limit as usize + 2];
    for i in 0..len / 2 {
        diff[2] += 2;
        diff[(2 * limit + 1) as usize] += 1;
        let a = nums[i];
        let b = nums[len - i - 1];
        diff[1 + a.min(b) as usize] -= 1;
        diff[(limit + a.max(b) + 1) as usize] += 1;
        diff[(a + b) as usize] -= 1;
        diff[(a + b + 1) as usize] += 1;
    }
    let mut result = len as i32;
    let mut cursum = 0;
    for i in 2..(2 * limit as usize + 2) {
        cursum += diff[i];
        result = result.min(cursum);
    }
    result
}

fn main() {
    assert_eq!(min_moves(vec![20744, 7642, 19090, 9992, 2457, 16848, 3458, 15721], 22891), 4);
    assert_eq!(min_moves(vec![1, 2, 4, 3], 4), 1);
    assert_eq!(min_moves(vec![1, 2, 2, 1], 2), 2);
    assert_eq!(min_moves(vec![1, 2, 1, 2], 2), 0);
}
