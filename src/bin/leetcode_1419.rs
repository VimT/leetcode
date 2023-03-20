//! 数青蛙

pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let mut croak = [0; 5];
    let mut result = 0;
    for &ch in croak_of_frogs.as_bytes() {
        let idx = match ch {
            b'c' => 0,
            b'r' => 1,
            b'o' => 2,
            b'a' => 3,
            b'k' => 4,
            _ => { return -1; }
        };
        if idx == 0 {
            if croak[4] > 0 {
                croak[4] -= 1;
            } else {
                result += 1;
            }
        } else {
            croak[idx - 1] -= 1;
        }
        croak[idx] += 1;
        if croak.iter().any(|x| *x < 0) { return -1; }
    }
    if croak[..4].iter().all(|x| *x == 0) {
        return result;
    }
    -1
}

fn main() {
    fn test(func: fn(croak_of_frogs: String) -> i32) {
        assert_eq!(func(String::from("croakcroak")), 1);
        assert_eq!(func(String::from("aoocrrackk")), -1);
        assert_eq!(func(String::from("crcoakroak")), 2);
        assert_eq!(func(String::from("croakcrook")), -1);
    }
    test(min_number_of_frogs);
}
