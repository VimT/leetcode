//! 设计停车系统

struct ParkingSystem {
    left: [i32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self { left: [big, medium, small] }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let n = &mut self.left[car_type as usize - 1];
        if *n > 0 {
            *n -= 1;
            true
        } else { false }
    }
}


fn main() {
    let mut ps = ParkingSystem::new(1, 1, 0);
    assert_eq!(ps.add_car(1), true); // 返回 true ，因为有 1 个空的大车位
    assert_eq!(ps.add_car(2), true); // 返回 true ，因为有 1 个空的中车位
    assert_eq!(ps.add_car(3), false); // 返回 false ，因为没有空的小车位
    assert_eq!(ps.add_car(1), false); // 返回 false ，因为没有空的大车位，唯一一个大车位已经被占据了
}
