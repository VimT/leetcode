//! 中心对称数 II

pub fn find_strobogrammatic(n: i32) -> Vec<String> {
    if n == 1 {
        return vec![String::from("0"), String::from("1"), String::from("8")];
    }
    if n == 2 {
        return vec![String::from("11"), String::from("69"), String::from("88"), String::from("96")];
    }
    let mut p1 = vec![String::from("0"), String::from("1"), String::from("8")];
    let mut p2 = vec![String::from("00"), String::from("11"), String::from("69"), String::from("88"), String::from("96")];
    for i in 3..=n {
        let mut tmp = vec![];
        for s in p1 {
            if i != n {
                tmp.push(String::from("0") + &s + "0");
            }
            tmp.push(String::from("1") + &s + "1");
            tmp.push(String::from("6") + &s + "9");
            tmp.push(String::from("8") + &s + "8");
            tmp.push(String::from("9") + &s + "6");
        }
        p1 = p2;
        p2 = tmp;
    }
    p2
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<String>) {
        assert_eq!(func(3), vec!["101", "609", "808", "906", "111", "619", "818", "916", "181", "689", "888", "986"]);
        assert_eq!(func(2), vec!["11", "69", "88", "96"]);
        assert_eq!(func(1), vec!["0", "1", "8"]);
    }
    test(find_strobogrammatic);
}
