//! 账户合并

use std::collections::HashMap;

use leetcode::{svec, unorder};
use leetcode::union_find::UnionFind;

pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    // id - emails map
    let mut emails: Vec<Vec<String>> = vec![vec![]; accounts.len()];
    // email - id map
    let mut ei: HashMap<&String, usize> = HashMap::new();
    let mut uf = UnionFind::new(accounts.len());
    for (i, account) in accounts.iter().enumerate() {
        for email in &account[1..] {
            match ei.get(email) {
                None => {
                    ei.insert(email, i);
                }
                Some(&v) => {
                    us.union(v, i);
                }
            }
        }
    }
    for (email, idx) in ei {
        let group = us.find(idx);
        emails[group].push(email.clone());
    }
    let mut ans = vec![];
    for (i, mut email) in emails.into_iter().enumerate() {
        if !email.is_empty() {
            let mut tmp = vec![];
            tmp.push(accounts[i][0].clone());
            email.sort_unstable();
            tmp.extend_from_slice(&email);
            ans.push(tmp);
        }
    }
    ans
}


fn main() {
    assert_eq!(unorder(accounts_merge(vec![svec!["John","johnsmith@mail.com","john_newyork@mail.com"], svec!["John","johnsmith@mail.com","john00@mail.com"], svec!["Mary","mary@mail.com"], svec!["John","johnnybravo@mail.com"]])), unorder(vec![svec!["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"], svec!["Mary","mary@mail.com"], svec!["John","johnnybravo@mail.com"]]));
    assert_eq!(unorder(accounts_merge(vec![svec!["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"], svec!["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"], svec!["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"], svec!["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"], svec!["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]])), unorder(vec![svec!["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"], svec!["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"], svec!["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"], svec!["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"], svec!["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]));
}
