//! 得分最高的最小轮调

/// 每个数的坏区间是固定的，算出每个坏区间，更新差分数组，最后差分还原
pub fn best_rotation(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut bad = vec![0; len];
    for i in 0..len {
        let left = (i as i32 - nums[i] + 1 + len as i32) % len as i32;
        let right = (i + 1) % len;
        bad[left as usize] -= 1;
        bad[right] += 1;
        if left as usize > right {
            bad[0] -= 1;
        }
    }
    let mut result = 0;
    let mut cur = 0;
    let mut best = -(len as i32);
    for i in 0..len {
        cur += bad[i];
        if cur > best {
            best = cur;
            result = i;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(best_rotation(vec![2, 3, 1, 4, 0]), 3);
    assert_eq!(best_rotation(vec![1, 3, 0, 2, 4]), 0);
}
