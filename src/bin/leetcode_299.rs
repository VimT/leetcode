//! 猜数字游戏

pub fn get_hint(secret: String, guess: String) -> String {
    let secret = secret.as_bytes();
    let guess = guess.as_bytes();
    let len = secret.len();
    let mut scnt = [0; 10];
    let mut gcnt = [0; 10];
    let mut a = 0;
    let mut b = 0;
    for i in 0..len {
        if secret[i] == guess[i] {
            a += 1;
        } else {
            scnt[(secret[i] - b'0') as usize] += 1;
            gcnt[(guess[i] - b'0') as usize] += 1;
        }
    }
    for i in 0..10 {
        b += scnt[i].min(gcnt[i]);
    }
    format!("{}A{}B", a, b)
}


fn main() {
    assert_eq!(get_hint(String::from("1807"), String::from("7810")), String::from("1A3B"));
    assert_eq!(get_hint(String::from("1123"), String::from("0111")), String::from("1A1B"));
}
