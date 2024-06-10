//! 找到连续赢 K 场比赛的第一位玩家

pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let len = skills.len();
    let mut win = vec![0; len];
    let mut cur_win = 0;
    for i in 1..len {
        if skills[cur_win] > skills[i] {
            win[cur_win] += 1;
        } else {
            cur_win = i;
            win[cur_win] += 1;
        }
        if win[cur_win] == k {
            return cur_win as i32;
        }
    }
    cur_win as i32
}

fn main() {
    fn test(func: fn(skills: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![4, 2, 6, 3, 9], 2), 2);
        assert_eq!(func(vec![2, 5, 4], 3), 1);
    }
    test(find_winning_player);
}
