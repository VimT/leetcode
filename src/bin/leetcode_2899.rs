//! 上一个遍历的整数

pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
    let mut s = vec![];
    let mut result = vec![];
    let mut prev = 0;
    for word in words {
        if word == "prev" {
            prev += 1;
            result.push(if prev > s.len() { -1 } else { s[s.len() - prev] });
        } else {
            prev = 0;
            s.push(word.parse::<i32>().unwrap());
        }
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(svec!["1","2","prev","prev","prev"]), vec![2, 1, -1]);
        assert_eq!(func(svec!["1","prev","2","prev","prev"]), vec![1, 2, 1]);
    }
    test(last_visited_integers);
}
