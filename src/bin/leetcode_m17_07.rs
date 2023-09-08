//! baby-names-lcci


use std::collections::HashMap;
use leetcode::svec;
use leetcode::union_find::UnionFind;

pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
    let len = names.len();
    let max_len = synonyms.len() * 2;
    let mut name_idx_map = HashMap::with_capacity(max_len);
    let mut name_frq_map = HashMap::with_capacity(max_len);
    let mut ns = Vec::with_capacity(max_len);
    for (idx, name_frq) in names.iter().enumerate() {
        let sp = name_frq.find("(").unwrap();
        let name = name_frq[..sp].to_string();
        let frq = name_frq[sp + 1..name_frq.len() - 1].parse::<i64>().unwrap();
        name_idx_map.insert(name.clone(), idx);
        name_frq_map.insert(name.clone(), frq);
        ns.push(name);
    }

    for synonym in synonyms.iter() {
        let sp = synonym.find(",").unwrap();
        let name1 = synonym[1..sp].to_string();
        let name2 = synonym[sp + 1..synonym.len() - 1].to_string();
        if !name_idx_map.contains_key(&name1) {
            name_idx_map.insert(name1.clone(), ns.len());
            name_frq_map.insert(name1.clone(), 0);
            ns.push(name1.clone());
        }
        if !name_idx_map.contains_key(&name2) {
            name_idx_map.insert(name2.clone(), ns.len());
            name_frq_map.insert(name2.clone(), 0);
            ns.push(name2.clone());
        }
    }
    let mut uf = UnionFind::new(ns.len());

    for synonym in synonyms {
        let sp = synonym.find(",").unwrap();
        let name1 = synonym[1..sp].to_string();
        let name2 = synonym[sp + 1..synonym.len() - 1].to_string();

        let idx1 = *name_idx_map.get(&name1).unwrap();
        let idx2 = *name_idx_map.get(&name2).unwrap();

        let parent1 = us.find(idx1);
        let parent2 = us.find(idx2);
        let mut min_idx = idx1;
        if name2 < ns[min_idx] { min_idx = idx2; }
        if ns[parent1] < ns[min_idx] { min_idx = parent1; }
        if ns[parent2] < ns[min_idx] { min_idx = parent2; }
        us.union_force(min_idx, idx1);
        us.union_force(min_idx, idx2);
    }
    let mut ans = vec![0; len];
    for (idx, name) in ns.iter().enumerate() {
        let frq = name_frq_map.get(name).unwrap();
        let parent = us.find(idx);
        ans[parent] += frq;
    }
    let mut real_ans = vec![];
    for (idx, frq) in ans.into_iter().enumerate() {
        if frq > 0 {
            real_ans.push(format!("{}({})", ns[idx], frq));
        }
    }
    real_ans
}

fn main() {
    assert_eq!(truly_most_popular(
        svec!["John(15)", "Jon(12)", "Chris(13)", "Kris(4)", "Christopher(19)"],
        svec!["(Jon,John)", "(John,Johnny)", "(Chris,Kris)", "(Chris,Christopher)"]), ["John(27)", "Chris(36)"]);
    assert_eq!(truly_most_popular(
        svec!["a(10)", "c(13)"],
        svec!["(a,b)", "(c,d)", "(b,c)"]), ["a(23)"]);
}
