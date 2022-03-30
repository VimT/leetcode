//! 从给定原材料中找到所有可以做出的菜

use std::collections::{HashMap, HashSet};

use leetcode::svec;

pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
    let mut indegree: HashMap<&str, i32> = HashMap::new();
    let mut edge: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut supp = HashSet::new();
    for i in supplies {
        supp.insert(i);
    }
    let len = recipes.len();
    for i in 0..len {
        let mut ind = 0;
        for need in &ingredients[i] {
            if !supp.contains(need) {
                edge.entry(need.as_str()).or_default().push(recipes[i].as_str());
                ind += 1;
            }
        }
        indegree.insert(recipes[i].as_str(), ind);
    }
    let mut q = vec![];
    for (k, v) in &indegree {
        if *v == 0 {
            q.push(*k);
        }
    }
    let mut result = vec![];
    while !q.is_empty() {
        let node = q.pop().unwrap();
        result.push(node.to_string());
        if let Some(targets) = edge.get(node) {
            for &target in targets {
                if let Some(v) = indegree.get_mut(target) {
                    if *v > 0 {
                        *v -= 1;
                        if *v == 0 {
                            q.push(target);
                        }
                    }
                }
            }
        }
    }

    result
}

fn main() {
    assert_eq!(find_all_recipes(svec!["bread"], vec![svec!["yeast", "flour"]], svec!["yeast", "flour", "corn"]), svec!["bread"]);
    assert_eq!(find_all_recipes(svec!["bread", "sandwich"], vec![svec!["yeast", "flour"], svec!["bread", "meat"]], svec!["yeast", "flour", "meat"]), svec!["bread", "sandwich"]);
    assert_eq!(find_all_recipes(svec!["bread", "sandwich", "burger"], vec![svec!["yeast", "flour"], svec!["bread", "meat"], svec!["sandwich", "meat", "bread"]], svec!["yeast", "flour", "meat"]), svec!["bread", "sandwich", "burger"]);
    assert_eq!(find_all_recipes(svec!["bread"], vec![svec!["yeast", "flour"]], svec!["yeast"]).is_empty(), true);
}
