//! 长按键入

pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let s = name.as_bytes();
    let t = typed.as_bytes();
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else {
            if i > 0 && t[j] == s[i - 1] {
                while j < t.len() && t[j] == s[i - 1] {
                    j += 1;
                }
            } else {
                return false;
            }
        }
    }
    if i == s.len() && j < t.len() {
        while j < t.len() && t[j] == s[i - 1] { j += 1; }
    }
    i == s.len() && j == t.len()
}

fn main() {
    assert_eq!(is_long_pressed_name(String::from("vtkgn"), String::from("vttkgnn")), true);
    assert_eq!(is_long_pressed_name(String::from("alex"), String::from("aaleex")), true);
    assert_eq!(is_long_pressed_name(String::from("saeed"), String::from("ssaaedd")), false);
}
