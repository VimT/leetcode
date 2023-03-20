//! 收藏清单


use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    let len = favorite_companies.len();
    let mut bi: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    let f: Vec<Vec<usize>> = favorite_companies.into_iter().map(|x| {
        let mut xx: Vec<usize> = x.into_iter().map(|y| {
            match bi.entry(y) {
                Entry::Occupied(v) => { *v.get() }
                Entry::Vacant(v) => {
                    v.insert(i);
                    i += 1;
                    i - 1
                }
            }
        }).collect();
        xx.sort_unstable();
        xx
    }).collect();
    fn contain(a: &Vec<usize>, b: &Vec<usize>) -> bool {
        let mut i = 0;
        let mut j = 0;
        let mut m = 0;
        while i < a.len() && j < b.len() {
            if a[i] < b[j] {
                i += 1;
            } else if a[i] > b[j] {
                j += 1;
            } else {
                m += 1;
                i += 1;
                j += 1;
            }
        }
        m == b.len()
    }
    let mut result = vec![];
    let mut is_other = vec![false; len]; // i是别人的子集
    for i in 0..len {
        let mut ok = true;
        for j in 0..len {
            if i != j && !is_other[j] {
                if contain(&f[j], &f[i]) {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            result.push(i as i32);
        } else {
            is_other[i] = true;
        }
    }

    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(favorite_companies: Vec<Vec<String>>) -> Vec<i32>) {
        assert_eq!(func(vec![svec!["leetcode","google","facebook"], svec!["google","microsoft"], svec!["google","facebook"], svec!["google"], svec!["amazon"]]), vec![0, 1, 4]);
        assert_eq!(func(vec![svec!["leetcode","google","facebook"], svec!["leetcode","amazon"], svec!["facebook","google"]]), vec![0, 1]);
        assert_eq!(func(vec![svec!["leetcode"], svec!["google"], svec!["facebook"], svec!["amazon"]]), vec![0, 1, 2, 3]);
    }
    test(people_indexes);
}
