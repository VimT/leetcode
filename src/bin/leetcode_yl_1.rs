//! 银联-01. 回文链表

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut v = vec![];
    let mut p = &head;
    while let Some(pn) = p {
        v.push(pn.val);
        p = &pn.next;
    }
    fn check(v: &[i32]) -> bool {
        let mut l = 0;
        let mut r = v.len() - 1;
        while l < r {
            if v[l] != v[r] { return false; }
            l += 1;
            r -= 1;
        }
        true
    }
    let len = v.len();
    let mut diff = false;
    for i in 0..len / 2 {
        if v[i] != v[len - 1 - i] {
            if check(&v[i + 1..=len - 1 - i]) {
                return true;
            } else if check(&v[i..len - 1 - i]) {
                return true;
            }
            diff = true;
            break;
        }
    }
    return !diff;
}

fn main() {
    assert_eq!(is_palindrome(link![1, 2, 2, 3, 1]), true);
    assert_eq!(is_palindrome(link![5, 1, 8, 8, 1, 5]), true);
    assert_eq!(is_palindrome(link![1, 2, 3, 4]), false);
}
