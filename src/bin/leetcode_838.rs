//! 推多米诺

pub fn push_dominoes(mut dominoes: String) -> String {
    let s = unsafe { dominoes.as_bytes_mut() };
    let len = s.len();
    let mut last_right = -1;
    let mut last_left = -1;
    for i in 0..=len {
        if i == len || s[i] == b'R' {
            if last_right > last_left {
                for j in (last_right + 1) as usize..i {
                    s[j] = b'R';
                }
            }
            last_right = i as i32;
        } else if s[i] == b'L' {
            if last_left >= last_right {
                for j in (last_left + 1) as usize..i {
                    s[j] = b'L';
                }
            } else if i > 0 {
                let mut l = (last_right + 1) as usize;
                let mut r = i - 1;
                while l < r {
                    s[l] = b'R';
                    s[r] = b'L';
                    l += 1;
                    r -= 1;
                }
            }
            last_left = i as i32;
        }
    }
    dominoes
}

fn main() {
    assert_eq!(push_dominoes(String::from(".L.R.")), String::from("LL.RR"));
    assert_eq!(push_dominoes(String::from("RR.L")), String::from("RR.L"));
    assert_eq!(push_dominoes(String::from(".L.R...LR..L..")), String::from("LL.RR.LLRRLL.."));
}
