//! 竖直打印单词

pub fn print_vertically(s: String) -> Vec<String> {
    let split: Vec<_> = s.split(' ').collect();
    let max_len = split.iter().map(|x| x.len()).max().unwrap();
    let mut result = vec![vec![b' '; split.len()]; max_len];
    for (i, chunk) in split.into_iter().enumerate() {
        for (j, &ch) in chunk.as_bytes().iter().enumerate() {
            result[j][i] = ch;
        }
    }
    unsafe {
        result.into_iter().map(|mut x| {
            while *x.last().unwrap() == b' ' { x.pop(); }
            String::from_utf8_unchecked(x)
        }).collect()
    }
}

fn main() {
    fn test(func: fn(s: String) -> Vec<String>) {
        assert_eq!(func(String::from("HOW ARE YOU")), vec!["HAY", "ORO", "WEU"]);
        assert_eq!(func(String::from("TO BE OR NOT TO BE")), vec!["TBONTB", "OEROOE", "   T"]);
        assert_eq!(func(String::from("CONTEST IS COMING")), vec!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]);
    }
    test(print_vertically);
}
