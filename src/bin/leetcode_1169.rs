//! 查询无效交易

use std::collections::HashMap;

use leetcode::svec;

pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    let mut result = vec![];
    for tran in transactions {
        let mut split = tran.split(",");
        let name = split.next().unwrap().to_string();
        let time: i32 = split.next().unwrap().parse().unwrap();
        let amount: i32 = split.next().unwrap().parse().unwrap();
        let city = split.next().unwrap().to_string();
        map.entry(name).or_insert(vec![]).push((time, amount, city, tran));
    }
    for (_, mut v) in map {
        v.sort_unstable();
        for (time, amount, city, tran) in &v {
            if *amount > 1000 {
                result.push(tran.clone());
            } else {
                let left = v.binary_search_by_key(&(time - 60), |x| x.0).unwrap_or_else(|x| x);
                let right = v.binary_search_by_key(&(time + 60), |x| x.0).map(|x| x + 1).unwrap_or_else(|x| x);
                for i in left..right {
                    if &v[i].2 != city {
                        result.push(tran.clone());
                        break;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(transactions: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["alice,20,800,mtv","alice,50,100,beijing"]), vec!["alice,20,800,mtv", "alice,50,100,beijing"]);
        assert_eq!(func(svec!["alice,20,800,mtv","alice,50,1200,mtv"]), vec!["alice,50,1200,mtv"]);
        assert_eq!(func(svec!["alice,20,800,mtv","bob,50,1200,mtv"]), vec!["bob,50,1200,mtv"]);
    }
    test(invalid_transactions);
}
