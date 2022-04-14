//! 花园的最大总美丽值

pub fn maximum_beauty(mut flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
    flowers.sort_unstable();
    let len = flowers.len();
    let mut left = new_flowers;

    // 从右向左 找到所有new_flowers 都填到target，最多能填到什么位置
    let mut p = len;
    while p > 0 {
        if flowers[p - 1] < target {
            if left < (target - flowers[p - 1]) as i64 {
                break;
            }
            left -= (target - flowers[p - 1]) as i64;
        }
        p -= 1;
    }

    let mut cur_sum = flowers[0] as i64;
    let mut min_pos = 0;
    let mut result = 0;
    // p向右移，比那里
    for i in p..=len {
        while min_pos + 1 < i && flowers[min_pos + 1] as i64 * (min_pos + 2) as i64 - cur_sum - flowers[min_pos + 1] as i64 <= left {
            min_pos += 1;
            cur_sum += flowers[min_pos] as i64;
        }
        // 左边的 对齐到 min_pos后，多出来的均匀撒，最后得到最小花
        let cur_min = (left + cur_sum) / (min_pos + 1) as i64;
        result = result.max((len - i) as i64 * full as i64 + if i > 0 { cur_min.min(target as i64 - 1) * partial as i64 } else { 0 });
        if i == len || flowers[i] >= target {
            break;
        }
        left += (target - flowers[i]) as i64;
    }

    result
}

fn main() {
    fn test(func: fn(flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64) {
        assert_eq!(func(vec![20, 1, 15, 17, 10, 2, 4, 16, 15, 11], 2, 20, 10, 2), 14);
        assert_eq!(func(vec![13], 18, 15, 9, 2), 28);
        assert_eq!(func(vec![2, 4, 5, 3], 10, 5, 2, 6), 30);
        assert_eq!(func(vec![1, 3, 1, 1], 7, 6, 12, 1), 14);
    }
    test(maximum_beauty);
}
