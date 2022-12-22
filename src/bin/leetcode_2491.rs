//! 划分技能点相等的团队

pub fn divide_players(mut skill: Vec<i32>) -> i64 {
    skill.sort_unstable();
    let len = skill.len();
    let target = skill[0] + skill[len - 1];
    let mut result = 0;
    for i in 0..len / 2 {
        if skill[i] + skill[len - i - 1] != target {
            return -1;
        }
        result += (skill[i] * skill[len - i - 1]) as i64;
    }
    result
}

fn main() {
    fn test(func: fn(skill: Vec<i32>) -> i64) {
        assert_eq!(func(vec![3, 2, 5, 1, 3, 4]), 22);
        assert_eq!(func(vec![3, 4]), 12);
        assert_eq!(func(vec![1, 1, 2, 3]), -1);
    }
    test(divide_players);
}
