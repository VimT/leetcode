//! 跳跃游戏

/// 挨个遍历，更新可以到达的最右位置
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut right_most = 0;
    for i in 0..nums.len() {
        if i <= right_most {
            right_most = right_most.max(i + nums[i] as usize);
            if right_most >= nums.len() - 1 { return true; }
        }
    }
    false
}

fn main() {
    assert_eq!(can_jump(vec![0, 1]), false);
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
}

