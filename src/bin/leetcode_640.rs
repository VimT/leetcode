//! 求解方程

pub fn solve_equation(equation: String) -> String {
    fn parse(s: &[u8]) -> (i32, i32) {
        let mut x = 0;
        let mut num = 0;
        let mut start = 0;
        let len = s.len();
        while start < len {
            let mut next = start + 1;
            while next < len && s[next] != b'-' && s[next] != b'+' { next += 1; }
            let mut end = next;
            let mut this = 0;
            let mut neg = false;
            if s[start] == b'-' {
                neg = true;
            }
            if !s[start].is_ascii_digit() { start += 1; }
            let mut is_x = false;
            if s[end - 1] == b'x' {
                end -= 1;
                is_x = true;
            }
            for i in start..end {
                this = this * 10 + (s[i] - b'0') as i32;
            }
            if start >= end && is_x { this = 1; }
            if neg { this = -this; }
            if is_x { x += this; } else { num += this; }
            start = next;
        }
        (x, num)
    }
    let mut split = equation.split('=');
    let (mut left_x, left_num) = parse(split.next().unwrap().as_bytes());
    let (right_x, mut right_num) = parse(split.next().unwrap().as_bytes());
    left_x -= right_x;
    right_num -= left_num;
    if left_x == 0 {
        return if right_num == 0 {
            String::from("Infinite solutions")
        } else {
            String::from("No solution")
        };
    }
    return "x=".to_string() + &(right_num / left_x).to_string();
}

fn main() {
    assert_eq!(solve_equation(String::from("x+5-3+x=6+x-2")), String::from("x=2"));
    assert_eq!(solve_equation(String::from("x=x")), String::from("Infinite solutions"));
    assert_eq!(solve_equation(String::from("2x=x")), String::from("x=0"));
    assert_eq!(solve_equation(String::from("x=x+2")), String::from("No solution"));
}
