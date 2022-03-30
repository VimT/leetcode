//! 独特的电子邮件地址

use std::collections::HashSet;

use leetcode::svec;

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut email_set = HashSet::new();
    for email in emails {
        let (mut name, domain) = email.split_once('@').unwrap();
        if let Some(split) = name.find('+') {
            name = &name[..split];
        }
        let new_name = name.replace('.', "");
        email_set.insert(format!("{}@{}", new_name, domain));
    }
    email_set.len() as i32
}

fn main() {
    assert_eq!(num_unique_emails(svec!["test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"]), 2);
    assert_eq!(num_unique_emails(svec!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]), 3);
}
