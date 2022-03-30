//! Dota2 参议院

pub fn predict_party_victory(senate: String) -> String {
    let mut s = senate.into_bytes();
    let len = s.len();
    let mut banr = 0;
    let mut band = 0;
    loop {
        let mut r = 0;
        let mut d = 0;
        for i in 0..len {
            let ch = s[i];
            if ch == b'R' {
                if banr > 0 {
                    banr -= 1;
                    s[i] = b'_';
                } else {
                    r += 1;
                    band += 1;
                }
            }
            if ch == b'D' {
                if band > 0 {
                    band -= 1;
                    s[i] = b'_';
                } else {
                    d += 1;
                    banr += 1;
                }
            }
        }
        if r == 0 {
            return String::from("Dire");
        }
        if d == 0 {
            return String::from("Radiant");
        }
    }
}

fn main() {
    assert_eq!(predict_party_victory(String::from("DRRD")), String::from("Dire"));
    assert_eq!(predict_party_victory(String::from("RDD")), String::from("Dire"));
    assert_eq!(predict_party_victory(String::from("DR")), String::from("Dire"));
    assert_eq!(predict_party_victory(String::from("RD")), String::from("Radiant"));
}
