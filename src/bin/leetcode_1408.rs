//! 数组中的字符串匹配

pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for (i, a) in words.iter().enumerate() {
        for (j, b) in words.iter().enumerate() {
            if i != j {
                if b.contains(a) {
                    result.push(a.clone());
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    use leetcode::svec;
    use leetcode::unorder;
    fn test(func: fn(words: Vec<String>) -> Vec<String>) {
        assert_eq!(unorder(func(svec!["mass","as","hero","superhero"])), unorder(vec!["as", "hero"]));
        assert_eq!(unorder(func(svec!["leetcode","et","code"])), unorder(vec!["et", "code"]));
        assert_eq!(unorder(func(svec!["blue","green","bu"])), unorder::<String>(vec![]));
    }
    test(string_matching);
}
