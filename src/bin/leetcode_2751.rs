//! 机器人碰撞

pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
    let mut bots: Vec<(i32, i32, u8, usize)> = positions.into_iter().zip(healths).zip(directions.into_bytes()).enumerate().map(|(i, x)| (x.0.0, x.0.1, x.1, i)).collect();
    bots.sort_unstable();
    let mut stk = vec![];
    let mut result = vec![];
    for (_, mut health, d, i) in bots {
        if d == b'L' {
            while let Some((j, l)) = stk.pop() {
                if l >= health {
                    if l > health { stk.push((j, l - 1)) };
                    health = 0;
                    break;
                } else {
                    health -= 1;
                }
            }
            if health > 0 {
                result.push((i, health));
            }
        } else {
            stk.push((i, health));
        }
    }
    result.extend_from_slice(&stk);
    result.sort_unstable();
    result.into_iter().map(|x| x.1).collect()
}

fn main() {
    fn test(func: fn(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32>) {
        assert_eq!(func(vec![5, 4, 3, 2, 1], vec![2, 17, 9, 15, 10], String::from("RRRRR")), vec![2, 17, 9, 15, 10]);
        assert_eq!(func(vec![3, 5, 2, 6], vec![10, 10, 15, 12], String::from("RLRL")), vec![14]);
        assert_eq!(func(vec![1, 2, 5, 6], vec![10, 10, 11, 11], String::from("RLRL")), vec![]);
    }
    test(survived_robots_healths);
}
