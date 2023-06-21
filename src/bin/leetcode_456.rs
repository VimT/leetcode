//! 132 模式


use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

/// 枚举132的1，用单调栈维护 3和2 的关系： pop出来的是2，在栈里的是3
pub fn find132pattern(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 3 { return false; }
    let mut s = vec![];
    let mut max = i32::MIN;
    for i in (0..len).rev() {
        if nums[i] < max {
            // stack top is 3, max is 2, nums[i] is 1
            return true;
        }
        while !s.is_empty() && *s.last().unwrap() < nums[i] {
            let pop = s.pop().unwrap();
            max = max.max(pop);
        }
        // cur nums[i] is 3, stack poped is 2, continue iter to find 1
        s.push(nums[i]);
    }
    false
}

pub fn find132pattern2(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 3 { return false; }
    let mut s = vec![];
    let mut pre_min = vec![nums[0]];
    for i in 1..len {
        while !s.is_empty() && nums[*s.last().unwrap()] <= nums[i] {
            s.pop().unwrap();
        }
        if !s.is_empty() && pre_min[*s.last().unwrap()] < nums[i] {
            // stack top is 3, pre_min[top] is 1, nums[i] is 2
            return true;
        }
        s.push(i);
        pre_min.push(nums[i].min(*pre_min.last().unwrap()));
    }
    false
}

pub fn find132pattern3(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 3 { return false; }
    let mut tree: BTreeMap<i32, i32> = BTreeMap::new();
    for i in 1..len {
        *tree.entry(nums[i]).or_default() += 1;
    }
    let mut cur_min = nums[0];
    for i in 1..len - 1 {
        if let Entry::Occupied(mut v) = tree.entry(nums[i]) {
            *v.get_mut() -= 1;
            if *v.get() == 0 {
                v.remove();
            }
        }
        if cur_min + 1 < nums[i] {
            if let Some((&n2, _)) = tree.range(cur_min + 1..nums[i]).next() {
                if n2 < nums[i] {
                    return true;
                }
            }
        }
        cur_min = cur_min.min(nums[i]);
    }
    false
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 4]), false);
        assert_eq!(func(vec![3, 1, 4, 2]), true);
        assert_eq!(func(vec![-1, 3, 2, 0]), true);
    }
    test(find132pattern);
    test(find132pattern2);
    test(find132pattern3);
}
