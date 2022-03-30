//! 强密码检验器

pub fn strong_password_checker(password: String) -> i32 {
    let mut has_digit = false;
    let mut has_lower = false;
    let mut has_upper = false;
    let p = password.as_bytes();
    let len = p.len();
    let mut index = 0;
    let mut modify_count = 0;
    let mut cnt_mod = [0; 3];
    while index < len {
        let char = p[index];
        match char {
            b'0'..=b'9' => has_digit = true,
            b'a'..=b'z' => has_lower = true,
            b'A'..=b'Z' => has_upper = true,
            _ => {}
        }
        let mut i = index + 1;
        while i < len && p[i] == char { i += 1; }
        let l = i - index;
        if l >= 3 {
            modify_count += l / 3;
            cnt_mod[l % 3] += 1;
        }
        index = i;
    }
    /* 缺少的字符类型数目, 下界 */
    let n_missing_ctype = !has_digit as i32 + !has_upper as i32 + !has_lower as i32;
    /* 过短，插入缺少的字符数量 */
    if len < 6 {
        return n_missing_ctype.max(6 - len as i32);
    }
    /* 长度合法，修改去除连续子串 */
    if len <= 20 {
        return n_missing_ctype.max(modify_count as i32);
    }
    /* 过长，还可以删除 len - 20 个字符 */
    let mut remove_count = len - 20;
    /* 3n 型子串无法完全变为 3n+2 型，
    每个需要 1 次删除，
    只能把 n_remove 个变为 3n+2 型
    减少 n_remove 次后续修改
    */
    if remove_count < cnt_mod[0] {
        return n_missing_ctype.max((modify_count - remove_count) as i32) + len as i32 - 20;
    }

    /* 3n 型全部变为 3n+2 型 */
    remove_count -= cnt_mod[0];
    modify_count -= cnt_mod[0];


    /* 3n+1 型无法完全变为 3n+2 型，
        每个需要 2 次删除，
        减少 n_remove/2 次后续修改
        */
    if remove_count < cnt_mod[1] * 2 {
        return n_missing_ctype.max((modify_count - remove_count / 2) as i32) + len as i32 - 20;
    }

    remove_count -= cnt_mod[1] * 2;
    modify_count -= cnt_mod[1];

    return n_missing_ctype.max((modify_count - remove_count / 3) as i32) + len as i32 - 20;
}

fn main() {
    assert_eq!(strong_password_checker(String::from("a")), 5);
    assert_eq!(strong_password_checker(String::from("aA1")), 3);
    assert_eq!(strong_password_checker(String::from("1337C0d3")), 0);
}
