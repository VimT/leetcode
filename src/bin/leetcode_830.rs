//! 较大分组的位置

pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut cnt = 1;
    let s = s.as_bytes();
    let len = s.len();
    let mut ch = s[0];
    let mut result = vec![];
    for i in 1..=len {
        if i < len && s[i] == ch {
            cnt += 1;
        } else {
            if cnt >= 3 {
                result.push(vec![(i - cnt) as i32, (i - 1) as i32]);
            }
            if i < len {
                ch = s[i];
                cnt = 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(large_group_positions(String::from("abbxxxxzzy")), vec![vec![3, 6]]);
    assert_eq!(large_group_positions(String::from("abc")).is_empty(), true);
    assert_eq!(large_group_positions(String::from("abcdddeeeeaabbbcd")), vec![vec![3, 5], vec![6, 9], vec![12, 14]]);
}
