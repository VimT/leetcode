//! 对 Bob 造成的最少伤害

/// 只对比两个，可以推到出公式
/// 有两个敌人：(damage1, cost1) 和 (damage2, cost2)
/// 先消灭 a ，总伤害为：damage1 * cost1 + damage2 * (cost1 + cost2)
/// 先消灭 b ，总伤害为：damage2 * cost2 + damage1 * (cost1 + cost2)
/// 如果 a 比 b 更优，那么应该有
/// damage1 * cost1 + damage2 * (cost1 + cost2) < damage2 * cost2 + damage1 * (cost1 + cost2)
/// 化简得到： damage2 * cost1 < damage1 * cost2
pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
    let mut result = 0;
    let cost: Vec<i64> = health.into_iter().map(|h| ((h + power - 1) / power) as i64).collect();
    let mut a: Vec<(i64, i64)> = damage.into_iter().map(|x| x as i64).zip(cost).collect();
    a.sort_unstable_by(|(d1, c1), (d2, c2)| (d2 * c1).cmp(&(d1 * c2)));
    let mut seconds = 0;
    for (h, c) in a {
        seconds += c;
        result += h * seconds;
    }
    result
}

fn main() {
    fn test(func: fn(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64) {
        assert_eq!(func(40, vec![31, 22, 54], vec![29, 92, 53]), 325);
        assert_eq!(func(2, vec![82, 80, 60], vec![51, 65, 48]), 11772);
        assert_eq!(func(62, vec![80, 79], vec![86, 13]), 319);
        assert_eq!(func(4, vec![1, 2, 3, 4], vec![4, 5, 6, 8]), 39);
        assert_eq!(func(1, vec![1, 1, 1, 1], vec![1, 2, 3, 4]), 20);
        assert_eq!(func(8, vec![40], vec![59]), 320);
    }
    test(min_damage);
}
