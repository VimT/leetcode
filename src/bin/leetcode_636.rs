//! 函数的独占时间

use leetcode::svec;

pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let logs: Vec<(usize, bool, usize)> = logs.into_iter().map(|x| {
        let mut split = x.split(':');
        let func_id: usize = split.next().unwrap().parse().unwrap();
        let start = split.next().unwrap() == "start";
        let mut timestamp: usize = split.next().unwrap().parse().unwrap();
        if !start { timestamp += 1; }
        (func_id, start, timestamp)
    }).collect();
    let mut funcs = vec![0; n as usize];
    let len = logs.len();
    let mut s: Vec<usize> = vec![];
    for i in 0..len {
        let start = logs[i].1;
        if start {
            if !s.is_empty() {
                funcs[logs[*s.last().unwrap()].0] += logs[i].2 - logs[i - 1].2;
            }
            s.push(i);
        } else {
            if logs[i - 1].1 {
                //prev is start
                assert_eq!(logs[i - 1].0, logs[i].0);
                funcs[logs[i].0] += logs[i].2 - logs[i - 1].2;
            } else {
                funcs[logs[*s.last().unwrap()].0] += logs[i].2 - logs[i - 1].2;
            }
            s.pop().unwrap();
        }
    }
    funcs.into_iter().map(|x| x as i32).collect()
}

fn main() {
    assert_eq!(exclusive_time(1, svec!["0:start:0", "0:start:2", "0:end:5", "0:start:6", "0:end:6", "0:end:7"]), vec![8]);
    assert_eq!(exclusive_time(2, svec!["0:start:0", "1:start:2", "1:end:5", "0:end:6"]), vec![3, 4]);
    assert_eq!(exclusive_time(2, svec!["0:start:0", "0:start:2", "0:end:5", "1:start:6", "1:end:6", "0:end:7"]), vec![7, 1]);
}
