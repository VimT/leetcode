//! 找到离给定两个节点最近的节点

pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    let len = edges.len();
    let mut dis1 = vec![i32::MAX; len];
    let mut dis2 = vec![i32::MAX; len];
    fn bfs(edges: &Vec<i32>, start: i32, dis: &mut Vec<i32>) {
        let mut cur = start;
        let mut d = 0;
        while cur >= 0 && dis[cur as usize] == i32::MAX {
            dis[cur as usize] = d;
            d += 1;
            cur = edges[cur as usize];
        }
    }
    bfs(&edges, node1, &mut dis1);
    bfs(&edges, node2, &mut dis2);
    let mut min = i32::MAX;
    let mut result = -1;
    for i in 0..len {
        if dis1[i] != i32::MAX && dis2[i] != i32::MAX {
            let dis = dis1[i].max(dis2[i]);
            if dis < min {
                min = dis;
                result = i as i32;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(edges: Vec<i32>, node1: i32, node2: i32) -> i32) {
        assert_eq!(func(vec![5, 3, 1, 0, 2, 4, 5], 3, 2), 3);
        assert_eq!(func(vec![2, 2, 3, -1], 0, 1), 2);
        assert_eq!(func(vec![1, 2, -1], 0, 2), 2);
    }
    test(closest_meeting_node);
}
