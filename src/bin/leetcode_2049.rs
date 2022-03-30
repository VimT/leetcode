//! 统计最高分的节点数目

pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
    let len = parents.len();
    let mut left = vec![0; len];
    let mut right = vec![0; len];
    let mut node = vec![(0, 0); len];
    for i in 0..len {
        if parents[i] < 0 { continue; }
        let n = &mut node[parents[i] as usize];
        if n.0 == 0 {
            n.0 = i;
        } else {
            n.1 = i;
        }
    }
    fn inner(node: &Vec<(usize, usize)>, cur: usize, left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
        let left_cnt = if node[cur].0 > 0 { inner(node, node[cur].0, left, right) } else { 0 };
        let right_cnt = if node[cur].1 > 0 { inner(node, node[cur].1, left, right) } else { 0 };
        left[cur] = left_cnt;
        right[cur] = right_cnt;
        return left_cnt + right_cnt + 1;
    }
    inner(&node, 0, &mut left, &mut right);
    let mut max = 0;
    let mut max_cnt = 0;
    for i in 0..len {
        let left_cnt = left[i];
        let right_cnt = right[i];
        let parent_cnt = len as i32 - 1 - left_cnt - right_cnt;
        let mut mul: i64 = 1;
        if left_cnt > 0 { mul *= left_cnt as i64 }
        if right_cnt > 0 { mul *= right_cnt as i64; }
        if parent_cnt > 0 { mul *= parent_cnt as i64; }
        if mul == max {
            max_cnt += 1;
        } else if mul > max {
            max = mul;
            max_cnt = 1;
        }
    }
    max_cnt
}

fn main() {
    assert_eq!(count_highest_score_nodes(vec![-1, 2, 0, 2, 0]), 3);
    assert_eq!(count_highest_score_nodes(vec![-1, 2, 0]), 2);
}
