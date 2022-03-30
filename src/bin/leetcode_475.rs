//! 供暖器


pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    heaters.sort_unstable();
    let mut result = 0;
    for house in houses {
        let dis = match heaters.binary_search(&house) {
            Ok(_) => { 0 }
            Err(x) => {
                if x == 0 { heaters[x] - house } else if x == heaters.len() { house - heaters[x - 1] } else { (house - heaters[x - 1]).min(heaters[x] - house) }
            }
        };
        result = result.max(dis);
    }
    result
}

pub fn find_radius_double_point(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
    let mut result = 0;
    let len = houses.len();
    let hlen = heaters.len();
    let mut j = 0;
    for i in 0..len {
        let mut dis = (houses[i] - heaters[j]).abs();
        while j < hlen - 1 && (houses[i] - heaters[j]).abs() >= (houses[i] - heaters[j + 1]).abs() {
            j += 1;
            dis = dis.min((houses[i] - heaters[j]).abs());
        }
        result = result.max(dis);
    }
    result
}

fn main() {
    assert_eq!(find_radius_double_point(vec![1, 1, 1, 1, 1, 1, 999, 999, 999, 999, 999], vec![499, 500, 501]), 498);
    assert_eq!(find_radius_double_point(vec![1], vec![1, 2, 3, 4]), 0);
    assert_eq!(find_radius_double_point(vec![1, 5], vec![10]), 9);
    assert_eq!(find_radius_double_point(vec![1, 2, 3, 4], vec![1, 4]), 1);
    assert_eq!(find_radius_double_point(vec![1, 2, 3], vec![2]), 1);
    assert_eq!(find_radius_double_point(vec![1, 5], vec![2]), 3);
}
