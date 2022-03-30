//! 警告一小时内使用相同员工卡大于等于三次的人


use leetcode::svec;

enum Status {
    History(Vec<u32>),
    Alarm,
}

pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
    let mut map = std::collections::HashMap::new();
    let len = key_name.len();
    let mut result = vec![];
    for i in 0..len {
        let name = &key_name[i];
        let time = &key_time[i];
        let split_time: Vec<u32> = time.split(":").map(|x| x.parse::<u32>().unwrap()).collect();
        let int_time = split_time[0] * 60 + split_time[1];
        let status = map.entry(name).or_insert(Status::History(vec![]));
        match status {
            Status::History(history) => {
                let idx = history.binary_search(&int_time).unwrap_or_else(|x| x);
                history.insert(idx, int_time);
                let len = history.len();
                if (idx >= 2 && (history[idx] - history[idx - 2] <= 60)) || (idx >= 1 && idx + 1 < len && (history[idx + 1] - history[idx - 1] <= 60)) || (idx + 2 < len && history[idx + 2] - history[idx] <= 60) {
                    *status = Status::Alarm;
                    result.push(name.clone());
                }
            }
            Status::Alarm => continue
        }
    }
    result.sort_unstable();
    result
}


fn main() {
    assert_eq!(alert_names(svec!["daniel","daniel","daniel","luis","luis","luis","luis"], svec!["10:00","10:40","11:00","09:00","11:00","13:00","15:00"]), vec!["daniel"]);
    assert_eq!(alert_names(svec!["alice","alice","alice","bob","bob","bob","bob"], svec!["12:01","12:00","18:00","21:00","21:20","21:30","23:00"]), vec!["bob"]);
}
