//! 除法求值

use std::collections::{HashMap, VecDeque};

use leetcode::svec;

pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut items = vec![];
    let mut m = HashMap::new();

    for eq in &equations {
        for item in eq {
            let item = item.as_bytes();
            if let None = m.get(item) {
                items.push(eq[0].as_bytes());
                m.insert(item, items.len() - 1);
            }
        }
    }
    let len = items.len();
    let mut dp = vec![vec![-1.0; len]; len];
    for i in 0..len {
        dp[i][i] = 1.0;
    }
    let mut q = VecDeque::new();
    for i in 0..equations.len() {
        let i1 = m[equations[i][0].as_bytes()];
        let i2 = m[equations[i][1].as_bytes()];
        dp[i1][i2] = values[i];
        dp[i2][i1] = 1. / values[i];
        q.push_back((i1, i2));
        q.push_back((i2, i1));
    }
    while !q.is_empty() {
        let (start, cur) = q.pop_front().unwrap();
        for i in 0..len {
            if dp[cur][i] != -1.0 && dp[start][i] == -1.0 {
                dp[start][i] = dp[start][cur] * dp[cur][i];
                q.push_back((start, i));
            }
        }
    }
    let mut result = Vec::with_capacity(queries.len());
    for query in queries {
        let i1 = m.get(query[0].as_bytes());
        let i2 = m.get(query[1].as_bytes());
        result.push(if i1.is_some() && i2.is_some() {
            dp[*i1.unwrap()][*i2.unwrap()]
        } else {
            -1.
        })
    }

    result
}

fn main() {
    assert_eq!(calc_equation(vec![svec!["a", "b"], svec!["e", "f"], svec!["b", "e"]], vec![2.0, 3.0, 4.0], vec![svec!["a", "f"]]), vec![24.0]);
    assert_eq!(calc_equation(vec![svec!["a", "b"], svec!["b", "c"]], vec![2.0, 3.0], vec![svec!["a", "c"], svec!["b", "a"], svec!["a", "e"], svec!["a", "a"], svec!["x", "x"]]), vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]);
    assert_eq!(calc_equation(vec![svec!["a", "b"], svec!["b", "c"], svec!["bc", "cd"]], vec![1.5, 2.5, 5.0], vec![svec!["a", "c"], svec!["c", "b"], svec!["bc", "cd"], svec!["cd", "bc"]]), vec![3.75000, 0.40000, 5.00000, 0.20000]);
    assert_eq!(calc_equation(vec![svec!["a", "b"]], vec![0.5], vec![svec!["a", "b"], svec!["b", "a"], svec!["a", "c"], svec!["x", "y"]]), vec![0.50000, 2.00000, -1.00000, -1.00000]);
}
