//! 餐厅过滤器

pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
    let mut r: Vec<_> = restaurants.into_iter().filter_map(|x| {
        let (vegan, price, distance) = (x[2], x[3], x[4]);
        if (if vegan_friendly == 1 { vegan == 1 } else { true }) && price <= max_price && distance <= max_distance {
            Some((-x[1], -x[0]))
        } else {
            None
        }
    }).collect();
    r.sort_unstable();
    r.into_iter().map(|x| -x.1).collect()
}

fn main() {
    fn test(func: fn(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 4, 1, 40, 10], vec![2, 8, 0, 50, 5], vec![3, 8, 1, 30, 4], vec![4, 10, 0, 10, 3], vec![5, 1, 1, 15, 1]], 1, 50, 10), vec![3, 1, 5]);
        assert_eq!(func(vec![vec![1, 4, 1, 40, 10], vec![2, 8, 0, 50, 5], vec![3, 8, 1, 30, 4], vec![4, 10, 0, 10, 3], vec![5, 1, 1, 15, 1]], 0, 50, 10), vec![4, 3, 2, 1, 5]);
        assert_eq!(func(vec![vec![1, 4, 1, 40, 10], vec![2, 8, 0, 50, 5], vec![3, 8, 1, 30, 4], vec![4, 10, 0, 10, 3], vec![5, 1, 1, 15, 1]], 0, 30, 3), vec![4, 5]);
    }
    test(filter_restaurants);
}
