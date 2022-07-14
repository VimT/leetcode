//! 咒语和药水的成功对数

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();
    spells.into_iter().map(|spell| {
        let mut left = 0;
        let mut right = potions.len();
        while left < right {
            let mid = (left + right) / 2;
            if potions[mid] as i64 * spell as i64 >= success {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        (potions.len() - left) as i32
    }).collect()
}


pub fn successful_pairs2(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut spells: Vec<(i32, usize)> = spells.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    spells.sort_unstable();
    potions.sort_unstable();
    let mut i = potions.len();
    let mut result = vec![0; spells.len()];
    for (spell, idx) in spells {
        while i > 0 && spell as i64 * potions[i - 1] as i64 >= success {
            i -= 1;
        }
        result[idx] = (potions.len() - i) as i32;
    }
    result
}

fn main() {
    fn test(func: fn(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32>) {
        assert_eq!(func(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7), vec![4, 0, 3]);
        assert_eq!(func(vec![3, 1, 2], vec![8, 5, 8], 16), vec![2, 0, 2]);
    }
    test(successful_pairs);
    test(successful_pairs2);
}
