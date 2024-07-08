//! 交替组 II

/// 滑动窗口
pub fn number_of_alternating_groups(c: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut colors = c.clone();
    colors.extend_from_slice(&c[..k - 1]);
    let len = colors.len();
    let mut cur_diff = 0;
    let mut result = 0;
    for i in 0..k - 1 {
        if colors[i] != colors[i + 1] { cur_diff += 1; }
    }
    if cur_diff + 1 == k { result += 1; }
    for i in k - 1..len - 1 {
        if colors[i] != colors[i + 1] { cur_diff += 1; }
        if colors[i + 1 - k] != colors[i + 2 - k] { cur_diff -= 1; }
        if cur_diff + 1 == k { result += 1; }
    }
    result
}

/// 另一种做法，统计当前连续 diff 的数量
pub fn number_of_alternating_groups2(c: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let len = c.len();
    let mut result = 0;
    let mut cnt = 0;
    for i in 0..len * 2 {
        if i > 0 && c[i % len] == c[(i + len - 1) % len] {
            cnt = 0;
        }
        cnt += 1;
        if i >= len && cnt >= k {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(colors: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![0, 1, 0, 1, 0], 3), 3);
        assert_eq!(func(vec![0, 1, 0, 0, 1, 0, 1], 6), 2);
        assert_eq!(func(vec![1, 1, 0, 1], 4), 0);
    }
    test(number_of_alternating_groups);
    test(number_of_alternating_groups2);
}
