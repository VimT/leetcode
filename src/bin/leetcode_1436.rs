//! 旅行终点站

use std::collections::HashMap;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut degree: HashMap<String, i32> = HashMap::new(); // 出度
    for path in paths {
        *degree.entry(path[0].clone()).or_default() += 1;
        degree.entry(path[1].clone()).or_default();
    }
    for (k, v) in degree {
        if v == 0 {
            return k;
        }
    }
    unreachable!()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(paths: Vec<Vec<String>>) -> String) {
        assert_eq!(func(vec![svec!["London","New York"], svec!["New York","Lima"], svec!["Lima","Sao Paulo"]]), String::from("Sao Paulo"));
        assert_eq!(func(vec![svec!["B","C"], svec!["D","B"], svec!["C","A"]]), String::from("A"));
        assert_eq!(func(vec![svec!["A","Z"]]), String::from("Z"));
    }
    test(dest_city);
}
