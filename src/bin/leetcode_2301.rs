//! 替换字符后匹配

pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
    let s = s.into_bytes();
    let sub = sub.as_bytes();
    let mut map = vec![vec![false; 128]; 128];
    for mapping in mappings {
        map[mapping[0] as usize][mapping[1] as usize] = true;
    }
    for s in s.windows(sub.len()) {
        let mut ok = true;
        for (&ach, &bch) in s.iter().zip(sub) {
            if ach != bch && !map[bch as usize][ach as usize] {
                ok = false;
                break;
            }
        }
        if ok { return true; }
    }
    false
}

fn main() {
    fn test(func: fn(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool) {
        assert_eq!(func(String::from("fool3e7bar"), String::from("leet"), vec![vec!['e', '3'], vec!['t', '7'], vec!['t', '8']]), true);
        assert_eq!(func(String::from("fooleetbar"), String::from("f00l"), vec![vec!['o', '0']]), false);
        assert_eq!(func(String::from("Fool33tbaR"), String::from("leetd"), vec![vec!['e', '3'], vec!['t', '7'], vec!['t', '8'], vec!['d', 'b'], vec!['p', 'b']]), true);
    }
    test(match_replacement);
}
