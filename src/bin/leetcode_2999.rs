//! 统计强大整数的数目

/// 数位DP
pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
    fn dfs(d @ (n1, n2, limit, s): (&Vec<i32>, &Vec<i32>, i32, &Vec<Option<i32>>), i: usize, min_limit: bool, max_limit: bool, mem: &mut Vec<i64>) -> i64 {
        if i == n2.len() { return 1; }
        if !min_limit && !max_limit && mem[i] != -1 {
            return mem[i];
        }
        let start = if min_limit { n1[i] } else { 0 };
        let end = (if max_limit { n2[i] } else { 9 }).min(limit);
        if let Some(ch) = s[i] {
            if ch < start || ch > end { return 0; }
            return dfs(d, i + 1, min_limit && ch == n1[i], max_limit && ch == n2[i], mem);
        }

        let mut result = 0;
        for ch in start..=end {
            result += dfs(d, i + 1, min_limit && ch == n1[i], max_limit && ch == n2[i], mem);
        }
        if !min_limit && !max_limit {
            mem[i] = result;
        }
        result
    }

    let mut n1: Vec<i32> = start.to_string().as_bytes().iter().map(|x| (*x - b'0') as i32).collect();
    let n2: Vec<i32> = finish.to_string().as_bytes().iter().map(|x| (*x - b'0') as i32).collect();
    let len = n2.len();
    while n1.len() < len { n1.insert(0, 0); }
    let mut s: Vec<Option<i32>> = s.as_bytes().iter().map(|x| Some((*x - b'0') as i32)).collect();
    while s.len() < len { s.insert(0, None); }
    dfs((&n1, &n2, limit, &s), 0, true, true, &mut vec![-1; len])
}

fn main() {
    fn test(func: fn(start: i64, finish: i64, limit: i32, s: String) -> i64) {
        assert_eq!(func(15, 215, 6, String::from("10")), 2);
        assert_eq!(func(1, 6000, 4, String::from("124")), 5);
        assert_eq!(func(1000, 2000, 4, String::from("3000")), 0);
    }
    test(number_of_powerful_int);
}
