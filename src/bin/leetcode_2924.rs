//! 找到冠军 II

pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut seen = vec![false; n];
    for edge in edges {
        seen[edge[1] as usize] = true;
    }
    let mut result = None;
    for i in 0..n {
        if !seen[i] {
            if result.is_some() { return -1; }
            result = Some(i as i32);
        }
    }
    result.unwrap()
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![vec![0, 1], vec![1, 2]]), 0);
        assert_eq!(func(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]), -1);
    }
    test(find_champion);
}
