//! 最大合金数

pub fn max_number_of_alloys(n: i32, _: i32, budget: i32, composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = n as usize;
    composition.into_iter().map(|m| {
        let mut left = 0;
        let mut right = 1000000000;
        while left < right {
            let mid = (left + right) / 2;
            let mut total_cost = 0;
            for i in 0..n {
                let need = m[i] as i64 * mid;
                if need > stock[i] as i64 {
                    total_cost += (need - stock[i] as i64) * cost[i] as i64;
                }
            }
            if total_cost > budget as i64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32 - 1
    }).max().unwrap()
}

fn main() {
    fn test(func: fn(n: i32, k: i32, budget: i32, composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32) {
        assert_eq!(func(3, 2, 15, vec![vec![1, 1, 1], vec![1, 1, 10]], vec![0, 0, 0], vec![1, 2, 3]), 2);
        assert_eq!(func(3, 2, 15, vec![vec![1, 1, 1], vec![1, 1, 10]], vec![0, 0, 100], vec![1, 2, 3]), 5);
        assert_eq!(func(2, 3, 10, vec![vec![2, 1], vec![1, 2], vec![1, 1]], vec![1, 1], vec![5, 5]), 2);
    }
    test(max_number_of_alloys);
}
