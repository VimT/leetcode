//! 使数组中所有元素相等的最小开销

/// 分类讨论：
/// 1. 当 cost1 * 2 <= cost2 || n <= 2 时，可以全部使用 cost1
/// 2. 其他情况，应该尽可能多的使用 cost2。那么变成类似 1953 的题
pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
    let len = nums.len();
    let (cost1, cost2) = (cost1 as i64, cost2 as i64);
    let mx = nums.iter().max().copied().unwrap() as i64;
    let mn = nums.iter().min().copied().unwrap() as i64;
    let mut sum: i64 = nums.iter().map(|&x| (mx - x as i64)).sum();
    let mx = mx - mn;
    const MOD: i64 = 1_000_000_007;
    if len <= 2 || cost1 * 2 <= cost2 {
        return (sum * cost1 % MOD) as i32;
    }
    let mut result = i64::MAX;
    // 枚举目标数
    for target in mx..=mx * 2 {
        let t = if target > sum - target {
            (sum - target) * cost2 + (target - sum + target) * cost1
        } else {
            sum / 2 * cost2 + (sum % 2) * cost1
        };
        result = result.min(t);
        sum += len as i64;
    }

    (result % MOD) as i32
}

/// 灵神优化：不用枚举 mx..=mx*2
pub fn min_cost_to_equalize_array2(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
    let len = nums.len() as i64;
    let (cost1, cost2) = (cost1 as i64, cost2 as i64);
    let mut mx = nums[0] as i64;
    let mut mn = nums[0] as i64;
    let mut sum = 0;
    for num in nums {
        let num = num as i64;
        mx = mx.max(num);
        mn = mn.min(num);
        sum += num;
    }

    let sum = mx * len - sum;
    const MOD: i64 = 1_000_000_007;
    if len <= 2 || cost1 * 2 <= cost2 {
        return (sum * cost1 % MOD) as i32;
    }
    let f = |x: i64| -> i64 {
        let s = sum + (x - mx) * len;
        let d = x - mn;
        if d * 2 <= s {
            s / 2 * cost2 + s % 2 * cost1
        } else {
            (s - d) * cost2 + (d * 2 - s) * cost1
        }
    };
    let i = (len * mx - mn * 2 - sum + len - 3) / (len - 2);
    (if i <= mx {
        f(mx).min(f(mx + 1))
    } else {
        f(mx).min(f(i - 1)).min(f(i)).min(f(i + 1))
    } % MOD) as i32
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32) {
        assert_eq!(func(vec![1, 14, 14, 15], 2, 1), 20);
        assert_eq!(func(vec![4, 1], 5, 2), 15);
        assert_eq!(func(vec![2, 3, 3, 3, 5], 2, 1), 6);
        assert_eq!(func(vec![3, 5, 3], 1, 3), 4);
    }
    test(min_cost_to_equalize_array);
    test(min_cost_to_equalize_array2);
}
