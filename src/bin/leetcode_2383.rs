//! 赢得比赛需要的最少训练时长

pub fn min_number_of_hours(mut initial_energy: i32, mut initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>) -> i32 {
    let mut need = 0;
    for (a, b) in energy.into_iter().zip(experience) {
        if initial_energy <= a {
            let diff = a + 1 - initial_energy;
            need += diff;
            initial_energy += diff;
        }
        if initial_experience <= b {
            let diff = b + 1 - initial_experience;
            need += diff;
            initial_experience += diff;
        }
        initial_experience += b;
        initial_energy -= a;
    }
    need
}

fn main() {
    fn test(func: fn(initial_energy: i32, initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>) -> i32) {
        assert_eq!(func(5, 3, vec![1, 4], vec![2, 5]), 2);
        assert_eq!(func(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]), 8);
        assert_eq!(func(2, 4, vec![1], vec![3]), 0);
    }
    test(min_number_of_hours);
}
