//! 在圆内随机生成点

use std::f64::consts::PI;

use rand::Rng;

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution { radius, x_center, y_center }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let du: f64 = rng.gen_range(0f64, 2f64 * PI);
        // 点落在半径为x的圆内概率为x^2，所以为了使点均匀分布，x=sqrt(U),
        let len: f64 = rng.gen::<f64>().sqrt() * self.radius;
        let x = len * du.cos();
        let y = len * du.sin();
        return vec![self.x_center + x, self.y_center + y];
    }
}

fn main() {
    let s = Solution::new(1_f64, 0_f64, 0.0);
    println!("{:?}", s.rand_point());
    println!("{:?}", s.rand_point());
    println!("{:?}", s.rand_point());
    let s = Solution::new(10.0, 5.0, -7.5);
    println!("{:?}", s.rand_point());
    println!("{:?}", s.rand_point());
    println!("{:?}", s.rand_point());
}