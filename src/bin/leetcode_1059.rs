//! 从始点到终点的所有路径

pub fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut map = vec![vec![]; n as usize];
    for edge in edges {
        map[edge[0] as usize].push(edge[1] as usize);
    }
    fn dfs(map: &Vec<Vec<usize>>, cur: usize, target: usize, seen: &mut Vec<bool>) -> bool {
        if cur == target {
            return true;
        }
        if map[cur].is_empty() {
            return false;
        }
        for &next in &map[cur] {
            if seen[next] {
                return false;
            }
            seen[next] = true;
            if !dfs(map, next, target, seen) {
                return false;
            }
            seen[next] = false;
        }
        true
    }
    if !map[destination as usize].is_empty() {
        return false;
    }
    let mut seen = vec![false; n as usize];
    seen[source as usize] = true;
    dfs(&map, source as usize, destination as usize, &mut seen)
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool) {
        assert_eq!(func(3, vec![vec![0, 1], vec![0, 2]], 0, 2), false);
        assert_eq!(func(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![2, 1]], 0, 3), false);
        assert_eq!(func(4, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]], 0, 3), true);
    }
    test(leads_to_destination);
}
