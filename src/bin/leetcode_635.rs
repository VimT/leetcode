//! 设计日志存储系统

use std::collections::BTreeMap;

struct LogSystem {
    log: BTreeMap<i32, i32>,
}

const MINUTE: i32 = 60;
const HOUR: i32 = MINUTE * 60;
const DAY: i32 = HOUR * 24;
const MONTH: i32 = DAY * 32;
const YEAR: i32 = MONTH * 13;
static MAX: [i32; 6] = [0, 12, 31, 23, 59, 59];

impl LogSystem {
    fn new() -> Self {
        Self { log: BTreeMap::new() }
    }

    fn time_str2int(ts: String, granularity: &str, is_end: bool) -> i32 {
        let mut split = ts.split(':');
        let year: i32 = split.next().unwrap().parse::<i32>().unwrap() - 2000;
        let month: i32 = split.next().unwrap().parse::<i32>().unwrap();
        let day: i32 = split.next().unwrap().parse::<i32>().unwrap();
        let hour: i32 = split.next().unwrap().parse::<i32>().unwrap();
        let minute: i32 = split.next().unwrap().parse::<i32>().unwrap();
        let second: i32 = split.next().unwrap().parse::<i32>().unwrap();
        let mut wei = [year, month, day, hour, minute, second];
        let start = match granularity {
            "Year" => 0,
            "Month" => 1,
            "Day" => 2,
            "Hour" => 3,
            "Minute" => 4,
            _ => 5
        };
        for i in start + 1..6 {
            wei[i] = if is_end { MAX[i] } else { 0 };
        }
        wei[0] * YEAR + wei[1] * MONTH + wei[2] * DAY + wei[3] * HOUR + wei[4] * MINUTE + wei[5]
    }

    fn put(&mut self, id: i32, timestamp: String) {
        self.log.insert(Self::time_str2int(timestamp, "", false), id);
    }

    fn retrieve(&self, start: String, end: String, granularity: String) -> Vec<i32> {
        let start = Self::time_str2int(start, &granularity, false);
        let end = Self::time_str2int(end, &granularity, true);
        self.log.range(start..=end).map(|x| *x.1).collect()
    }
}


fn main() {
    let mut ls = LogSystem::new();
    ls.put(1, String::from("2017:01:01:23:59:59"));
    ls.put(2, String::from("2017:01:01:22:59:59"));
    ls.put(3, String::from("2016:01:01:00:00:00"));

    // 返回 [3,2,1]，返回从 2016 年到 2017 年所有的日志。
    let r = ls.retrieve(String::from("2016:01:01:01:01:01"), String::from("2017:01:01:23:00:00"), String::from("Year"));
    assert_eq!(r, [3, 2, 1]);

    // 返回 [2,1]，返回从 Jan. 1, 2016 01:XX:XX 到 Jan. 1, 2017 23:XX:XX 之间的所有日志
    // 不返回日志 3 因为记录时间 Jan. 1, 2016 00:00:00 超过范围的起始时间
    let r = ls.retrieve(String::from("2016:01:01:01:01:01"), String::from("2017:01:01:23:00:00"), String::from("Hour"));
    assert_eq!(r, [2, 1]);
}
