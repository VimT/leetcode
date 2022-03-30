//! 大礼包

use std::collections::HashMap;

pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    let len = price.len();
    let special: Vec<Vec<i32>> = special.into_iter().filter(|x| {
        let mut total_cnt = 0;
        let mut total_price = 0;
        for i in 0..len {
            total_cnt += x[i];
            total_price += x[i] * price[i];
        }
        return total_cnt > 0 && total_price > x[len];
    }).collect();

    fn dfs(price: &Vec<i32>, special: &Vec<Vec<i32>>, need: Vec<i32>, cache: &mut HashMap<Vec<i32>, i32>) -> i32 {
        let len = price.len();
        if let Some(v) = cache.get(&need) {
            return *v;
        }
        let mut min_price = 0;
        for i in 0..len {
            min_price += need[i] * price[i];
        }
        'out: for sp in special {
            let sp_price = sp[len];
            let mut next_needs = vec![0; len];
            for i in 0..len {
                if sp[i] > need[i] {
                    continue 'out;
                }
                next_needs[i] = need[i] - sp[i];
            }
            min_price = min_price.min(dfs(price, special, next_needs, cache) + sp_price);
        }

        cache.insert(need, min_price);
        return min_price;
    }
    let mut cache = HashMap::new();
    dfs(&price, &special, needs, &mut cache)
}

fn main() {
    assert_eq!(shopping_offers(vec![2, 3, 4], vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]], vec![1, 2, 1]), 11);
    assert_eq!(shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]), 14);
}
