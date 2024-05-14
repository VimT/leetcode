//! 从魔法师身上吸取的最大能量

pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let len = energy.len() as i32;
    let mut result = i32::MIN;
    for start in 0..k {
        let mut p = len - 1 - start;
        let mut sum = 0;
        while p >= 0 {
            sum += energy[p as usize];
            p -= k;
            result = result.max(sum);
        }
    }
    result
}

fn main() {
    fn test(func: fn(energy: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![8, -5], 1), 3);
        assert_eq!(func(vec![-1, -2, -8, 6, -6, -6, -6, 5, 5], 8), 6);
        assert_eq!(func(vec![5, 2, -10, -5, 1], 3), 3);
        assert_eq!(func(vec![-2, -3, -1], 2), -1);
    }
    test(maximum_energy);
}
