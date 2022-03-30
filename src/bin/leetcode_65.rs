//! 有效数字

pub fn is_number(s: String) -> bool {
    let bytes = s.trim().as_bytes();
    let len = bytes.len();
    if len == 0 { return false; }
    let mut i = 0;
    if bytes[i] == b'+' || bytes[i] == b'-' { i += 1; }

    let mut in_decimal = false;
    let mut decimal_len = 0;
    let mut int_len = 0;
    let mut has_e = false;
    while i < len {
        let c = bytes[i];
        if c.is_ascii_digit() {
            if in_decimal { decimal_len += 1; } else { int_len += 1; }
            i += 1;
        } else if c == b'.' {
            if in_decimal { return false; }
            in_decimal = true;
            i += 1;
        } else if c == b'e' {
            i += 1;
            has_e = true;
            break;
        } else {
            return false;
        }
    }
    if int_len == 0 && decimal_len == 0 { return false; }
    let mut e_len = 0;
    if i < len && (bytes[i] == b'+' || bytes[i] == b'-') { i += 1; }
    while i < len {
        let c = bytes[i];
        if c.is_ascii_digit() {
            e_len += 1;
            i += 1;
        } else {
            return false;
        }
    }
    if has_e && e_len == 0 { return false; }

    true
}

fn main() {
    assert_eq!(is_number(". 1".to_string()), false);
    assert_eq!(is_number("3.".to_string()), true);
    assert_eq!(is_number("..2".to_string()), false);
    assert_eq!(is_number(".2".to_string()), true);
    assert_eq!(is_number("0".to_string()), true);
    assert_eq!(is_number(" ".to_string()), false);
    assert_eq!(is_number(" 0.1".to_string()), true);
    assert_eq!(is_number("abc".to_string()), false);
    assert_eq!(is_number("1 a".to_string()), false);
    assert_eq!(is_number("2e10".to_string()), true);
    assert_eq!(is_number(" -90e3".to_string()), true);
    assert_eq!(is_number(" 1e".to_string()), false);
    assert_eq!(is_number("e3".to_string()), false);
    assert_eq!(is_number(" 6e-1".to_string()), true);
    assert_eq!(is_number("99e2.5".to_string()), false);
    assert_eq!(is_number("53.5e93".to_string()), true);
    assert_eq!(is_number(" --6".to_string()), false);
    assert_eq!(is_number("-+3".to_string()), false);
    assert_eq!(is_number("95a54e53".to_string()), false);
}

