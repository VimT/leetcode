//! 可以到达所有点的最少点数目

pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut indegree = vec![0; n as usize];
    for edge in edges {
        indegree[edge[1] as usize] += 1;
    }
    indegree.into_iter().zip(0..).filter_map(|(d, i)| if d == 0 { Some(i) } else { None }).collect()
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(6, vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]), vec![0, 3]);
        assert_eq!(func(5, vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]]), vec![0, 2, 3]);
    }
    test(find_smallest_set_of_vertices);
}
