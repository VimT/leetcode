//! 两球之间的磁力

pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort_unstable();
    let mut left = 1;
    let mut right = *position.last().unwrap() - position[0];
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut cnt = 1;
        let mut last = position[0];
        for &p in &position[1..] {
            if p - last >= mid {
                cnt += 1;
                last = p;
            }
        }
        if cnt >= m {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(position: Vec<i32>, m: i32) -> i32) {
        assert_eq!(func(vec![269826447, 974181916, 225871443, 189215924, 664652743, 592895362, 754562271, 335067223, 996014894, 510353008, 48640772, 228945137], 3), 461712236);
        assert_eq!(func(vec![1, 2, 3, 4, 7], 3), 3);
        assert_eq!(func(vec![5, 4, 3, 2, 1, 1000000000], 2), 999999999);
    }
    test(max_distance);
}
