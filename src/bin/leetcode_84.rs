//! 柱状图中最大的矩形


use std::collections::VecDeque;

/// 暴力
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let len = heights.len();
    let mut ans = 0;
    for i in 1..=len {
        let mut min = heights[i - 1];
        for j in i..=len {
            min = min.min(heights[j - 1]);
            ans = ans.max(min * (j - i + 1) as i32)
        }
    }
    ans
}

/// 分治
pub fn largest_rectangle_area_div(heights: Vec<i32>) -> i32 {
    fn inner(heights: &Vec<i32>, start: usize, end: usize) -> i32 {
        if start > end { return 0; }
        let mut min_index = start;
        for i in start..=end {
            if heights[min_index] > heights[i] { min_index = i; }
        }
        (heights[min_index] * (end - start + 1) as i32).max(
            if min_index == 0 { 0 } else { inner(heights, start, min_index - 1) }).max(
            inner(heights, min_index + 1, end))
    }
    inner(&heights, 0, heights.len() - 1)
}

/// 单调栈
/// 递增栈操作： 如果元素比top大，则入栈，否则弹出直到元素比top大
/// 性质： 元素出栈时，即将入栈的元素 是出栈元素 右边第一个 比出栈元素 小的数
///       元素出栈后，栈顶元素是出栈元素左边第一个比其小的元素
/// 在本地中，利用单调栈的性质，出栈的元素总是最大的， 计算[出栈元素, 当前遍历点] 的面积。最后stack中剩余的元素一定是最小的
pub fn largest_rectangle_area_stack(mut heights: Vec<i32>) -> i32 {
    let mut stack = VecDeque::new();
    let mut ans = 0;
    heights.push(0);
    for i in 0..heights.len() {
        while !stack.is_empty() && heights[*stack.back().unwrap()] > heights[i] {
            let cur = stack.pop_back().unwrap();
            let left = stack.back().map_or(0, |x| *x + 1);
            let right = i - 1;
            ans = ans.max((right - left + 1) as i32 * heights[cur]);
        }
        stack.push_back(i);
    }
    ans
}


fn main() {
    fn test(func: fn(heights: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(func(vec![2, 4]), 4);
    }
    test(largest_rectangle_area);
    test(largest_rectangle_area_div);
    test(largest_rectangle_area_stack);
}
