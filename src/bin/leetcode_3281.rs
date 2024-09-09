//! 范围内整数的最大得分

pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
    let mut start: Vec<i64> = start.into_iter().map(|x| x as i64).collect();
    let d = d as i64;
    let len = start.len();
    start.sort_unstable();

    let mut left = 0;
    let mut right = start[len - 1] + d - start[0];

    while left < right {
        let mid = left + (right - left + 1) / 2; // 一次跳跃的最小距离
        let mut ok = true;
        let mut last = start[0];
        for i in 1..len {
            // 检查能不能从 last 跳到 start[i]
            if last + mid > start[i] + d {
                ok = false;
                break;
            }
            // 让 last 尽可能小
            last = start[i].max(last + mid);
        }

        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left as i32
}


fn main() {
    fn test(func: fn(start: Vec<i32>, d: i32) -> i32) {
        assert_eq!(func(vec![1000000000, 0], 1000000000), 2000000000);
        assert_eq!(func(vec![2, 0, 0, 8], 2), 2);
        assert_eq!(func(vec![6, 0, 3], 2), 4);
        assert_eq!(func(vec![2, 6, 13, 13], 5), 5);
    }
    test(max_possible_score);
}
