//! 表示数字的最少运算符

use std::collections::HashMap;

/// 每一个块都应该是 x 的次幂
pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
    let mut cache = HashMap::new();
    fn dfs(target: i32, x: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        // 贪心
        // 优先使用乘法
        // 直到乘到小于或等于target数字x^p,然后使用加法逼近target
        // 直到乘到大于target数字x^q,然后使用减法逼近target
        if target < x {
            return (target * 2 - 1).min((x - target) * 2);
        }
        if let Some(&ans) = cache.get(&target) {
            return ans;
        }
        let p = (target as f64).log(x as f64) as i32;
        let q = p + 1;
        let costp = if p > 0 { p - 1 } else { 1 };
        let costq = if q > 0 { q - 1 } else { 1 };
        // 如果tartget=x^p,那么最优解为p个x相乘,代价为costp
        let powp = x.pow(p as u32);
        if powp == target {
            return costp;
        }
        let dist1 = target - powp;
        // 对于x^p，使用加法逼近target，代价为costp+1+func(target-x^p)
        let mut result = i32::MAX;
        if dist1 < target {
            result = result.min(costp + 1 + dfs(dist1, x, cache));
        }
        if let Some(powq) = x.checked_pow(q as u32) {
            let dist2 = powq - target;
            // 对于x^q，使用减法逼近target，代价为q+func(x^q-target)
            if dist2 < target {
                result = result.min(costq + 1 + dfs(dist2, x, cache));
            }
        }
        cache.insert(target, result);
        result
    }
    dfs(target, x, &mut cache)
}

fn main() {
    assert_eq!(least_ops_express_target(3, 19), 5);
    assert_eq!(least_ops_express_target(5, 501), 8);
    assert_eq!(least_ops_express_target(100, 100000000), 3);
}
