//! 与敌人战斗后的最大分数

pub fn maximum_points(enemy_energies: Vec<i32>, cur: i32) -> i64 {
    let mn = enemy_energies.iter().min().copied().unwrap();
    let sum = enemy_energies.into_iter().map(|x| x as i64).sum::<i64>();
    if cur < mn { return 0; }
    (sum + cur as i64 - mn as i64) / mn as i64
}

fn main() {
    fn test(func: fn(enemy_energies: Vec<i32>, current_energy: i32) -> i64) {
        assert_eq!(func(vec![3, 2, 2], 2), 3);
        assert_eq!(func(vec![2], 10), 5);
    }
    test(maximum_points);
}
