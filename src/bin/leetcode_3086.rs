//! 拾起 K 个 1 需要的最少行动次数


/// 中位数贪心
pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
    let len = nums.len();
    let mut pos = vec![];
    let mut i = 0;
    let mut c = 0;  // nums 中连续 1 的最大长度
    while i < len {
        if nums[i] == 1 {
            let start = i;
            while i < len && nums[i] == 1 {
                pos.push(i);
                i += 1;
            }
            c = c.max(i - start);
        }
        i += 1;
    }

    let k = k as usize;
    let max_changes = max_changes as usize;
    c = c.min(k).min(3);
    if max_changes + c >= k {
        // 其余 k - c 个 1 可以全部用两次操作得到
        return ((k - c) * 2 + c) as i64 - (c != 0) as i64;
    }

    let len = pos.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + pos[i];
    }
    let mut result = usize::MAX;
    let size = k - max_changes;  // 其余的 1 只能一步一步挪
    for right in size..=len {
        let left = right - size;
        let mid = left + size / 2;
        let s1 = pos[mid] * (mid - left) - (presum[mid] - presum[left]);
        let s2 = presum[right] - presum[mid] - pos[mid] * (right - mid);
        result = result.min(s1 + s2); // [left, right) 每个 pos[i] 到 pos[mid] 的距离之和
    }
    (result + max_changes * 2) as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, max_changes: i32) -> i64) {
        assert_eq!(func(vec![1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 10, 2), 27);
        assert_eq!(func(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1), 3);
        assert_eq!(func(vec![0, 0, 0, 0], 2, 3), 4);
    }
    test(minimum_moves);
}
