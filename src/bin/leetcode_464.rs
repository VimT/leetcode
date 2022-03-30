//! 我能赢吗

/// 640ms
pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if desired_total <= 1 { return true; }
    if max_choosable_integer >= desired_total { return true; }
    if max_choosable_integer * (1 + max_choosable_integer) / 2 < desired_total { return false; }
    let mut dp = vec![0; 1 << max_choosable_integer as usize];
    for i in (0..1 << max_choosable_integer).rev() {
        let mut cnt = 0;
        let mut cur_sum = 0;
        for j in 0..max_choosable_integer {
            if i >> j & 1 == 1 {
                cnt += 1;
                cur_sum += j + 1;
                if cur_sum >= desired_total {
                    break;
                }
            }
        }
        let want = (cnt & 1) + 1;
        let last = if want == 2 { 1 } else { 2 };
        if cur_sum >= desired_total {
            dp[i] = last;
        } else {
            let mut win = false;
            for j in 0..max_choosable_integer {
                if i >> j & 1 == 0 {
                    if dp[i | 1 << j] == want {
                        win = true;
                        break;
                    }
                }
            }
            dp[i] = if win { want } else { last };
        }
    }
    dp[0] == 1
}

/// 60ms
pub fn can_i_win_dfs(max_choosable_integer: i32, desired_total: i32) -> bool {
    fn dfs(state: i32, len: i32, desired_total: i32, cache: &mut Vec<Option<bool>>) -> bool {
        if let Some(r) = cache[state as usize] {
            return r;
        }
        for i in 1..=len {
            let cur = 1 << (i - 1);
            if state & cur == 0 {
                if i >= desired_total || !dfs(cur | state, len, desired_total - i, cache) {
                    cache[state as usize] = Some(true);
                    return true;
                }
            }
        }
        cache[state as usize] = Some(false);
        false
    }
    if max_choosable_integer >= desired_total { return true; }
    if max_choosable_integer * (1 + max_choosable_integer) / 2 < desired_total { return false; }
    dfs(0, max_choosable_integer, desired_total, &mut vec![None; 1 << max_choosable_integer as usize])
}

fn main() {
    assert_eq!(can_i_win_dfs(5, 50), false);
    assert_eq!(can_i_win_dfs(10, 0), true);
    assert_eq!(can_i_win_dfs(10, 11), false);
    assert_eq!(can_i_win_dfs(18, 79), true);
    assert_eq!(can_i_win_dfs(10, 1), true);
}
