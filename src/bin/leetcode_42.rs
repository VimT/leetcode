//! 接雨水

use std::collections::VecDeque;

/// 一层一层计算，1400+ms
pub fn trap_layer(mut height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let len = height.len();
    loop {
        let mut left = len;
        let mut count = 0;
        for i in 0..len {
            if height[i] > 0 {
                if left == len {
                    left = i;
                } else {
                    ans += i - left - 1;
                    left = i;
                }
                count += 1;
            }
        }
        if left == len || count <= 1 {
            return ans as i32;
        }
        for i in &mut height {
            if *i > 0 {
                *i -= 1;
            }
        }
    }
}

/// 每个点向两边扩展，O(n2)
pub fn trap_double_expand(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let len = height.len();
    for i in 1..len - 1 {
        let mut left_max = 0;
        let mut right_max = 0;
        for j in (0..=i).rev() {
            left_max = left_max.max(height[j]);
        }
        for j in i..len {
            right_max = right_max.max(height[j]);
        }
        ans += left_max.min(right_max) - height[i];
    }
    ans
}

/// 数组保存 左边的最大值和右边的最大值
pub fn trap_dynamic(height: Vec<i32>) -> i32 {
    if height.len() == 0 { return 0; }
    let mut ans = 0;
    let len = height.len();
    let mut left_max = vec![height[0]];
    let mut right_max = vec![0; len];
    for i in 1..len {
        left_max.push(height[i].max(left_max[i - 1]));
    }
    right_max[len - 1] = height[len - 1];
    for i in (0..len - 1).rev() {
        right_max[i] = height[i].max(right_max[i + 1]);
    }
    for i in 1..len - 1 {
        ans += left_max[i].min(right_max[i]) - height[i];
    }
    ans
}

/// 栈
/// 当遍历墙的高度的时候，如果当前高度小于栈顶的墙高度，说明这里会有积水，我们将墙的高度的下标入栈。
/// 如果当前高度大于栈顶的墙的高度，说明之前的积水到这里停下，我们可以计算下有多少积水了。计算完，就把当前的墙继续入栈，作为新的积水的墙。
pub fn trap_stack(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
    for i in 0..height.len() {
        while !stack.is_empty() && height[i] > (*stack.back().unwrap()).1 {
            let (last, _) = stack.pop_back().unwrap();
            if stack.is_empty() { break; }
            let (ll, ll_height) = stack.back().unwrap();
            let distance = i - *ll - 1;
            let bounded_height = height[i].min(*ll_height) - height[last];
            ans += distance * bounded_height as usize;
        }
        stack.push_back((i, height[i]));
    }
    ans as i32
}

/// 双指针，对双最大数组的优化，一次遍历，空间复杂度O(1)
pub fn trap_double_point(height: Vec<i32>) -> i32 {
    if height.len() == 0 { return 0; }
    let mut ans = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }
    ans
}

fn main() {
    fn test(func: fn(height: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(func(vec![4, 2, 0, 3, 2, 5]), 9);
    }
    test(trap_double_expand);
    test(trap_dynamic);
    test(trap_double_point);
    test(trap_layer);
    test(trap_stack);
}
