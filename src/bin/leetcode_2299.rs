//! 强密码检验器 II

pub fn strong_password_checker_ii(password: String) -> bool {
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_number = false;
    let mut has_special = false;
    let mut last_ch = 0;
    for &ch in password.as_bytes() {
        if ch == last_ch { return false; }
        if ch.is_ascii_digit() { has_number = true; } else if ch.is_ascii_lowercase() { has_lower = true; } else if ch.is_ascii_uppercase() { has_upper = true; } else if matches!(ch, b'!'|b'@'|b'#'|b'$'|b'%'|b'^'|b'&'|b'*'|b'('|b')'|b'-'|b'+') { has_special = true; }
        last_ch = ch;
    }
    password.len() >= 8 && has_special && has_lower && has_upper && has_number
}

fn main() {
    fn test(func: fn(password: String) -> bool) {
        assert_eq!(func(String::from("IloveLe3tcode!")), true);
        assert_eq!(func(String::from("Me+You--IsMyDream")), false);
        assert_eq!(func(String::from("1aB!")), false);
    }
    test(strong_password_checker_ii);
}
