//! 平行课程


pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut indegree = vec![0; n];
    let mut g = vec![vec![]; n];
    for rela in relations {
        g[rela[0] as usize - 1].push(rela[1] as usize - 1);
        indegree[rela[1] as usize - 1] += 1;
    }
    let mut result = 0;
    let mut q = Vec::new();
    for i in 0..n {
        if indegree[i] == 0 {
            q.push(i);
        }
    }
    while !q.is_empty() {
        let mut nq = Vec::new();
        for u in q {
            for &v in &g[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    nq.push(v);
                }
            }
        }
        result += 1;
        q = nq;
    }
    for i in 0..n {
        // 有环
        if indegree[i] > 0 { return -1; }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, relations: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![vec![1, 3], vec![2, 3]]), 2);
        assert_eq!(func(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]), -1);
    }
    test(minimum_semesters);
}
