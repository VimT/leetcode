//! 查询带键的排列

pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    let mut p: Vec<i32> = (1..=m).collect();
    queries.into_iter().map(|q| {
        let pos = p.iter().position(|&x| x == q).unwrap();
        // remove+insert比自己swap快多了
        p.remove(pos);
        p.insert(0, q);
        pos as i32
    }).collect()
}

fn main() {
    fn test(func: fn(queries: Vec<i32>, m: i32) -> Vec<i32>) {
        assert_eq!(func(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
        assert_eq!(func(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
        assert_eq!(func(vec![7, 5, 5, 8, 3], 8), vec![6, 5, 0, 7, 5]);
    }
    test(process_queries);
}
