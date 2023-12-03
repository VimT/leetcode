//! 数学难题

pub fn math_problem(s: String) -> String {
    let mut a = 0;
    let mut b = 0;
    let mut i = 0;
    let s = s.as_bytes();
    let len = s.len();
    let mut right = false;
    while i < len {
        if s[i] == b'=' {
            right = true;
            i += 1;
            continue;
        }
        let mut num = 0;
        let mut pos = true;
        let mut has_digit = false;
        if s[i] == b'+' || s[i] == b'-' {
            pos = s[i] == b'+';
            i += 1;
        }
        if right { pos = !pos; }
        while i < len && s[i].is_ascii_digit() {
            has_digit = true;
            num = num * 10 + (s[i] - b'0') as i32;
            i += 1;
        }
        if i < len && s[i] == b'x' {
            if num == 0 && !has_digit { num += 1; }
            if pos { a += num; } else { a -= num; }
            i += 1;
        } else {
            if pos { b += num; } else { b -= num; }
        }
    }
    if a == 0 && b == 0 {
        return String::from("Infinite solutions");
    } else if a == 0 || b % a != 0 {
        return String::from("No solution");
    } else {
        return format!("x={}", -b / a);
    }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("x+5-3+x=6+x-2")), String::from("x=2"));
        assert_eq!(func(String::from("x=x")), String::from("Infinite solutions"));
        assert_eq!(func(String::from("x=x+2")), String::from("No solution"));
        assert_eq!(func(String::from("2x=x")), String::from("x=0"));
        assert_eq!(func(String::from("0x=0")), String::from("Infinite solutions"));
    }
    test(math_problem);
}
