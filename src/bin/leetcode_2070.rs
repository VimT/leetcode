//! 每一个查询的最大美丽值

pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let len = items.len();
    let mut max_beauti = vec![0; len];
    let mut max_price = 0;
    let mut min_price = i32::MAX;
    items.sort_unstable();
    let mut price = vec![0; len];
    let mut cur_beauti_max = 0;
    for i in 0..len {
        price[i] = items[i][0];
        max_price = max_price.max(items[i][0]);
        min_price = min_price.min(items[i][0]);
        cur_beauti_max = cur_beauti_max.max(items[i][1]);
        max_beauti[i] = cur_beauti_max;
    }
    let mut result = Vec::with_capacity(queries.len());
    for q in queries {
        if q < min_price {
            result.push(0);
        } else if q >= max_price {
            result.push(cur_beauti_max);
        } else {
            match price.binary_search(&q) {
                Ok(mut v) => {
                    while v < len && price[v] == q {
                        v += 1;
                    }
                    v -= 1;
                    result.push(max_beauti[v]);
                }
                Err(v) => {
                    result.push(max_beauti[v - 1]);
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(maximum_beauty(vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]], vec![1, 2, 3, 4, 5, 6]), vec![2, 4, 5, 5, 6, 6]);
    assert_eq!(maximum_beauty(vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]], vec![1]), vec![4]);
    assert_eq!(maximum_beauty(vec![vec![10, 1000]], vec![5]), vec![0]);
}
