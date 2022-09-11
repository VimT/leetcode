//! 将数组分成几个递增序列

use std::collections::BinaryHeap;

pub fn can_divide_into_subsequences(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let max_vec_num = len / k as usize;
    let mut groups = BinaryHeap::new();
    for _ in 0..max_vec_num {
        groups.push(0);
    }
    for num in nums {
        let min_num = -groups.pop().unwrap();
        if num > min_num {
            groups.push(-num);
        } else {
            return false;
        }
    }
    true
}

/// 题目已经是非递减了，所以其实是问连续出现的数字 能否分给k组
pub fn can_divide_into_subsequences2(nums: Vec<i32>, k: i32) -> bool {
    if k == 1 { return true; }
    let mut pre = nums[0];
    let mut cnt = 0;
    for i in 0..nums.len() {
        if pre == nums[i] {
            cnt += 1;
        } else {
            if cnt * k > nums.len() as i32 { return false; }
            pre = nums[i];
            cnt = 1;
        }
    }
    cnt * k <= nums.len() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> bool) {
        assert_eq!(func(vec![1, 2, 2, 3, 3, 4, 4], 3), true);
        assert_eq!(func(vec![5, 6, 6, 7, 8], 3), false);
    }
    test(can_divide_into_subsequences);
    test(can_divide_into_subsequences2);
}
