//! 连续差相同的数字

pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    fn dfs(cur: &mut Vec<i8>, cur_num: i32, n: usize, k: i8, result: &mut Vec<i32>) {
        if cur.len() == n {
            result.push(cur_num);
            return;
        }
        let last = *cur.last().unwrap();
        if last + k <= 9 {
            cur.push(last + k);
            dfs(cur, cur_num * 10 + (last + k) as i32, n, k, result);
            cur.pop();
        }
        if k != 0 && last - k >= 0 {
            cur.push(last - k);
            dfs(cur, cur_num * 10 + (last - k) as i32, n, k, result);
            cur.pop();
        }
    }
    let mut result = vec![];
    let mut cur = vec![];
    for i in 1..=9 {
        cur.push(i);
        dfs(&mut cur, i as i32, n as usize, k as i8, &mut result);
        cur.pop();
    }
    result
}

fn main() {
    assert_eq!(nums_same_consec_diff(2, 0), vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    assert_eq!(nums_same_consec_diff(3, 7), vec![181, 292, 707, 818, 929]);
    assert_eq!(nums_same_consec_diff(2, 1), vec![12, 10, 23, 21, 34, 32, 45, 43, 56, 54, 67, 65, 78, 76, 89, 87, 98]);
}
