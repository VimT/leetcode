//! K 件物品的最大和

pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, mut k: i32) -> i32 {
    let mut result = 0;
    if num_ones > 0 {
        result += num_ones.min(k);
        k -= result;
    }
    k -= num_zeros;
    if k > 0 {
        result -= k;
    }
    result
}

fn main() {
    fn test(func: fn(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32) {
        assert_eq!(func(3, 2, 0, 2), 2);
        assert_eq!(func(3, 2, 0, 4), 3);
    }
    test(k_items_with_maximum_sum);
}
