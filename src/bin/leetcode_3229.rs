//! 使数组等于目标数组所需的最少操作次数

/// 一段 sig 相同的数字，单调栈处理
pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let diff: Vec<i64> = target.into_iter().zip(nums).map(|(a, b)| (a - b) as i64).collect();

    let len = diff.len();
    let mut i = 0;
    let mut result = 0;
    while i < len {
        if diff[i] == 0 {
            i += 1;
            continue;
        }
        let sig = diff[i].signum();
        let mut s = vec![];
        while i < len && diff[i].signum() == sig {
            let mut need = true;
            while !s.is_empty() && s.last().unwrap() >= &diff[i].abs() {
                need = false;
                s.pop();
            }
            if need { result += diff[i].abs() - s.last().copied().unwrap_or(0); }
            s.push(diff[i].abs());
            i += 1;
        }
    }
    result
}

/// 差分数组
/// 差分数组相同，原数组也相同。
pub fn minimum_operations2(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let mut s = target[0] - nums[0];  // 表示 s，表示对 d1[i] 增大/减少的累积量：
    let mut result = s.abs() as i64;
    for (d1, d2) in nums.windows(2).map(|x| x[1] - x[0]).zip(target.windows(2).map(|x| x[1] - x[0])) {
        let k = d2 - d1;
        if k > 0 {
            result += (if s >= 0 { k } else { 0.max(k + s) }) as i64;
        } else {
            result -= (if s <= 0 { k } else { 0.min(k + s) }) as i64;
        }
        s += k;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: Vec<i32>) -> i64) {
        assert_eq!(func(vec![9, 2, 6, 10, 4, 8, 3, 4, 2, 3], vec![9, 5, 5, 1, 7, 9, 8, 7, 6, 5]), 20);
        assert_eq!(func(vec![1, 3, 2], vec![2, 1, 4]), 5);
        assert_eq!(func(vec![3, 5, 1, 2], vec![4, 6, 2, 4]), 2);
    }
    test(minimum_operations);
    test(minimum_operations2);
}
