//! K 个关闭的灯泡

/// 滑动窗口
pub fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
    let len = bulbs.len();
    let mut days = vec![0; len];
    for (i, &bulb) in bulbs.iter().enumerate() {
        days[bulb as usize - 1] = i as i32 + 1;
    }
    let mut result = i32::MAX;
    let mut left = 0;
    let k = k as usize;
    let mut right = k + 1;
    while right < len {
        let mut is_break = false;
        for i in left + 1..right {
            if days[i] < days[left] || days[i] < days[right] {
                left = i;
                right = i + k + 1;
                is_break = true;
                break;
            }
        }
        if !is_break {
            result = result.min(days[left].max(days[right]));
            left = right;
            right = right + k + 1;
        }
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(bulbs: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 3, 2], 1), 2);
        assert_eq!(func(vec![1, 2, 3], 1), -1);
    }
    test(k_empty_slots);
}
