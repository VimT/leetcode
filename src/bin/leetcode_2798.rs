//! 满足目标工作时长的员工数目

pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.into_iter().filter(|&x| x >= target).count() as i32
}

fn main() {
    fn test(func: fn(hours: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![0, 1, 2, 3, 4], 2), 3);
        assert_eq!(func(vec![5, 1, 4, 2, 2], 6), 0);
    }
    test(number_of_employees_who_met_target);
}
