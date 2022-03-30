//! 找出临界点之间的最小和最大距离
use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut pre = &head;
    let mut cur = &head.as_ref().unwrap().next;

    let mut points = vec![];
    let mut i = 0;
    while cur.is_some() && cur.as_ref().unwrap().next.is_some() {
        let nxt_val = cur.as_ref().unwrap().next.as_ref().unwrap().val;
        let pre_val = pre.as_ref().unwrap().val;
        let cur_val = cur.as_ref().unwrap().val;
        if cur_val > nxt_val && cur_val > pre_val {
            points.push(i);
        } else if cur_val < nxt_val && cur_val < pre_val {
            points.push(i);
        }
        i += 1;
        let nxt = &cur.as_ref().unwrap().next;
        pre = cur;
        cur = nxt;
    }

    if points.len() < 2 {
        return vec![-1, -1];
    }

    let max = points[points.len() - 1] - points[0];
    let mut min = max;
    for i in 1..points.len() {
        min = min.min(points[i] - points[i - 1]);
    }
    vec![min, max]
}

fn main() {
    assert_eq!(nodes_between_critical_points(link![3, 1]), vec![-1, -1]);
    assert_eq!(nodes_between_critical_points(link![5, 3, 1, 2, 5, 1, 2]), vec![1, 3]);
    assert_eq!(nodes_between_critical_points(link![1, 3, 2, 2, 3, 2, 2, 2, 7]), vec![3, 3]);
    assert_eq!(nodes_between_critical_points(link![2, 3, 3, 2]), vec![-1, -1]);
}