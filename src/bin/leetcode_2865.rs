//! 美丽塔 I

/// 枚举峰顶
pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let len = max_heights.len();
    let mut result = 0;
    for i in 0..len {
        let mut t = 0;
        let mut cur_min = max_heights[i];
        for i in (0..i).rev() {
            cur_min = cur_min.min(max_heights[i]);
            t += cur_min as i64;
        }
        cur_min = max_heights[i];
        for i in i..len {
            cur_min = cur_min.min(max_heights[i]);
            t += cur_min as i64;
        }

        result = result.max(t);
    }
    result
}

fn main() {
    fn test(func: fn(max_heights: Vec<i32>) -> i64) {
        assert_eq!(func(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(func(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(func(vec![3, 2, 5, 5, 2, 3]), 18);
    }
    test(maximum_sum_of_heights);
}
