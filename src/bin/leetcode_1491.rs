//! 去掉最低工资和最高工资后的工资平均值

pub fn average(mut salary: Vec<i32>) -> f64 {
    salary.sort_unstable();
    salary[1..salary.len() - 1].iter().sum::<i32>() as f64 / (salary.len() - 2) as f64
}

fn main() {
    fn test(func: fn(salary: Vec<i32>) -> f64) {
        assert_eq!(func(vec![4000, 3000, 1000, 2000]), 2500.00000);
        assert_eq!(func(vec![1000, 2000, 3000]), 2000.00000);
    }
    test(average);
}
