//! 给表达式添加运算符

use leetcode::unorder;

pub fn add_operators(num: String, target: i32) -> Vec<String> {
    fn inner(num: &[u8], target: i32, idx: usize, cur_str: &mut Vec<String>, cur: i64, pre_op: i64, mut cur_op: i64, ans: &mut Vec<String>) {
        if idx == num.len() {
            if cur == target as i64 && cur_op == 0 {
                ans.push(cur_str[1..].join(""));
            }
            return;
        }
        cur_op = cur_op * 10 + (num[idx] - b'0') as i64;
        if cur_op > 0 {
            inner(num, target, idx + 1, cur_str, cur, pre_op, cur_op, ans);
        }
        cur_str.push(String::from("+"));
        cur_str.push(cur_op.to_string());
        inner(num, target, idx + 1, cur_str, cur + cur_op, cur_op, 0, ans);
        cur_str.pop();
        cur_str.pop();
        if !cur_str.is_empty() {
            cur_str.push(String::from("-"));
            cur_str.push(cur_op.to_string());
            inner(num, target, idx + 1, cur_str, cur - cur_op, -cur_op, 0, ans);
            cur_str.pop();
            cur_str.pop();

            cur_str.push(String::from("*"));
            cur_str.push(cur_op.to_string());
            inner(num, target, idx + 1, cur_str, cur - pre_op + (cur_op * pre_op), cur_op * pre_op, 0, ans);
            cur_str.pop();
            cur_str.pop();
        }
    }
    let mut ans = vec![];
    inner(num.as_bytes(), target, 0, &mut vec![], 0, 0, 0, &mut ans);
    ans
}


fn main() {
    assert_eq!(unorder(add_operators(String::from("123"), 6)), vec!["1*2*3", "1+2+3"]);
    assert_eq!(unorder(add_operators(String::from("232"), 8)), vec!["2*3+2", "2+3*2"]);
    assert_eq!(unorder(add_operators(String::from("3456237490"), 9191)).is_empty(), true);
}
