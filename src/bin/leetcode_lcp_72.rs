//! 补给马车

pub fn supply_wagon(mut supplies: Vec<i32>) -> Vec<i32> {
    let len = supplies.len();
    let target = len / 2;
    while supplies.len() > target {
        let (_, idx) = supplies.windows(2).map(|w| w[0] + w[1]).zip(0..).min().unwrap();
        supplies[idx] += supplies[idx + 1];
        supplies.remove(idx + 1);
    }
    supplies
}

fn main() {
    fn test(func: fn(supplies: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![7, 3, 6, 1, 8]), [10, 15]);
        assert_eq!(func(vec![1, 3, 1, 5]), [5, 5]);
    }
    test(supply_wagon);
}
