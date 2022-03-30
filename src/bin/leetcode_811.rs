//! 子域名访问计数

use std::collections::HashMap;

use leetcode::{svec, unorder};

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut result = HashMap::new();
    for domain in cpdomains {
        let (cnt_str, domain) = domain.split_once(' ').unwrap();
        let cnt = cnt_str.parse::<i32>().unwrap();
        let split: Vec<&str> = domain.split('.').collect();
        for i in 0..split.len() {
            *result.entry(split[i..].join(".")).or_insert(0i32) += cnt;
        }
    }
    result.into_iter().map(|(k, v)| {
        format!("{} {}", v, k)
    }).collect()
}

fn main() {
    assert_eq!(unorder(subdomain_visits(svec!["9001 discuss.leetcode.com"])), unorder(svec!["9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"]));
    assert_eq!(unorder(subdomain_visits(svec!["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"])), unorder(svec!["901 mail.com", "50 yahoo.com", "900 google.mail.com", "5 wiki.org", "5 org", "1 intel.mail.com", "951 com"]));
}
