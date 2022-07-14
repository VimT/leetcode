//! 输出比赛匹配对

pub fn find_contest_match(n: i32) -> String {
    let mut result: Vec<String> = (1..=n).map(|x| x.to_string()).collect();
    while result.len() > 1 {
        let mut tmp = Vec::with_capacity(result.len() / 2);
        for i in 0..result.len() / 2 {
            tmp.push(format!("({},{})", result[i], result[result.len() - i - 1]));
        }
        result = tmp;
    }
    result.pop().unwrap()
}

fn main() {
    fn test(func: fn(n: i32) -> String) {
        assert_eq!(func(4), String::from("((1,4),(2,3))"));
        assert_eq!(func(8), String::from("(((1,8),(4,5)),((2,7),(3,6)))"));
    }
    test(find_contest_match);
}
