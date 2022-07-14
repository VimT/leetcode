//! 中心对称数 III

pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
    fn dfs(low: &str, high: &str, result: &mut i32, cur: String) {
        if cur.len() >= low.len() && cur.len() <= high.len() {
            if !((cur.len() == high.len() && cur.as_str() > high) || (cur.len() == low.len() && cur.as_str() < low) || (cur.len() >= 2 && cur.starts_with('0'))) {
                *result += 1;
            }
        }
        if cur.len() + 2 > high.len() { return; }
        dfs(low, high, result, String::from("0") + &cur + "0");
        dfs(low, high, result, String::from("1") + &cur + "1");
        dfs(low, high, result, String::from("6") + &cur + "9");
        dfs(low, high, result, String::from("8") + &cur + "8");
        dfs(low, high, result, String::from("9") + &cur + "6");
    }

    let mut result = 0;
    dfs(&low, &high, &mut result, String::from(""));
    dfs(&low, &high, &mut result, String::from("1"));
    dfs(&low, &high, &mut result, String::from("0"));
    dfs(&low, &high, &mut result, String::from("8"));
    result
}

fn main() {
    fn test(func: fn(low: String, high: String) -> i32) {
        assert_eq!(func(String::from("50"), String::from("100")), 3);
        assert_eq!(func(String::from("0"), String::from("0")), 1);
    }
    test(strobogrammatic_in_range);
}
