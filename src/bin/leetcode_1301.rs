//! 最大得分的路径数目

pub fn paths_with_max_score(mut board: Vec<String>) -> Vec<i32> {
    let len = board.len();
    let mut dp = vec![vec![[0, 0]; len]; len];
    const MOD: i32 = 1e9 as i32 + 7;
    dp[0][0][1] = 1;// 初始方案数
    static DIR: [(i32, i32); 3] = [(-1, 0), (0, -1), (-1, -1)];
    unsafe { board[len - 1].as_bytes_mut()[len - 1] = b'0'; }
    for i in 0..len {
        for j in 0..len {
            let ch = board[i].as_bytes()[j];
            if ch.is_ascii_digit() {
                let num = (ch - b'0') as i32;
                let mut score = 0;
                let mut plan = 0;
                for (dx, dy) in DIR {
                    let (px, py) = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
                    if px < len && py < len && dp[px][py][1] != 0 {
                        if dp[px][py][0] > score {
                            score = dp[px][py][0];
                            plan = dp[px][py][1];
                        } else if dp[px][py][0] == score {
                            plan += dp[px][py][1];
                        }
                    }
                }
                dp[i][j] = [score + num, plan % MOD];
            }
        }
    }
    dp[len - 1][len - 1].to_vec()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(board: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(svec!["E11345","X452XX","3X43X4","422812","284522","13422S"]), vec![34, 3]);
        assert_eq!(func(svec!["E11","XXX","11S"]), vec![0, 0]);
        assert_eq!(func(svec!["E23","2X2","12S"]), vec![7, 1]);
        assert_eq!(func(svec!["E12","1X1","21S"]), vec![4, 2]);
    }
    test(paths_with_max_score);
}
