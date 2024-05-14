//! 高访问员工

use std::collections::HashMap;

pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
    let mut m: HashMap<&str, Vec<i32>> = HashMap::new();
    for at in &access_times {
        let t = at[1][..2].parse::<i32>().unwrap() * 60 + at[1][2..].parse::<i32>().unwrap();
        m.entry(&at[0]).or_default().push(t);
    }
    let mut result = vec![];
    for (user, mut t) in m {
        if t.len() >= 3 {
            t.sort_unstable();
            for win in t.windows(3) {
                if win[2] - win[0] < 60 {
                    result.push(user.to_string());
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    use leetcode::unorder;
    use leetcode::svec;
    fn test(func: fn(access_times: Vec<Vec<String>>) -> Vec<String>) {
        assert_eq!(unorder(func(vec![svec!["a","0549"], svec!["b","0457"], svec!["a","0532"], svec!["a","0621"], svec!["b","0540"]])), vec!["a"]);
        assert_eq!(unorder(func(vec![svec!["d","0002"], svec!["c","0808"], svec!["c","0829"], svec!["e","0215"], svec!["d","1508"], svec!["d","1444"], svec!["d","1410"], svec!["c","0809"]])), vec!["c", "d"]);
        assert_eq!(unorder(func(vec![svec!["cd","1025"], svec!["ab","1025"], svec!["cd","1046"], svec!["cd","1055"], svec!["ab","1124"], svec!["ab","1120"]])), vec!["ab", "cd"]);
    }
    test(find_high_access_employees);
}
