//! 设计地铁系统

use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    in_station: HashMap<i32, (String, i32)>,
    analyse: HashMap<(String, String), (i64, i64)>,
}


impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.in_station.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (in_station, in_t) = self.in_station.remove(&id).unwrap();
        let (total, n) = self.analyse.entry((in_station, station_name)).or_default();
        *total += (t - in_t) as i64;
        *n += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let (total, n) = *self.analyse.get(&(start_station, end_station)).unwrap();
        total as f64 / n as f64
    }
}

fn main() {
    let mut uf = UndergroundSystem::new();
    us.check_in(45, String::from("Leyton"), 3);
    us.check_in(32, String::from("Paradise"), 8);
    us.check_in(27, String::from("Leyton"), 10);
    us.check_out(45, String::from("Waterloo"), 15);  // 乘客 45 "Leyton" -> "Waterloo" ，用时 15-3 = 12
    us.check_out(27, String::from("Waterloo"), 20);  // 乘客 27 "Leyton" -> "Waterloo" ，用时 20-10 = 10
    us.check_out(32, String::from("Cambridge"), 22); // 乘客 32 "Paradise" -> "Cambridge" ，用时 22-8 = 14
    assert_eq!(us.get_average_time(String::from("Paradise"), String::from("Cambridge")), 14.0); // 返回 14.00000 。只有一个 "Paradise" -> "Cambridge" 的行程，(14) / 1 = 14
    assert_eq!(us.get_average_time(String::from("Leyton"), String::from("Waterloo")), 11.0);    // 返回 11.00000 。有两个 "Leyton" -> "Waterloo" 的行程，(10 + 12) / 2 = 11
    us.check_in(10, String::from("Leyton"), 24);
    assert_eq!(us.get_average_time(String::from("Leyton"), String::from("Waterloo")), 11.0);    // 返回 11.00000
    us.check_out(10, String::from("Waterloo"), 38);  // 乘客 10 "Leyton" -> "Waterloo" ，用时 38-24 = 14
    assert_eq!(us.get_average_time(String::from("Leyton"), String::from("Waterloo")), 12.0);    // 返回 12.00000 。有三个 "Leyton" -> "Waterloo" 的行程，(10 + 12 + 14) / 3 = 12
}
