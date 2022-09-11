//! 会议室 III

use std::collections::BinaryHeap;

pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable();
    let mut available = BinaryHeap::new();
    for i in 0..n {
        available.push(-i);
    }
    let mut end: BinaryHeap<(i64, i32)> = BinaryHeap::new();
    let mut rooms = vec![0; n as usize];
    for meeting in meetings {
        while !end.is_empty() && -end.peek().unwrap().0 <= meeting[0] as i64 {
            let (_, room) = end.pop().unwrap();
            available.push(room);
        }
        if let Some(room) = available.pop() {
            rooms[-room as usize] += 1;
            end.push((-meeting[1] as i64, room));
        } else {
            let (end_time, room) = end.pop().unwrap();
            rooms[-room as usize] += 1;
            end.push((-meeting[1] as i64 - (-end_time - meeting[0] as i64), room));
        }
    }
    let mut result = 0;
    let mut max = 0;
    for i in 0..n as usize {
        if rooms[i] > max {
            result = i;
            max = rooms[i];
        }
    }
    result as i32
}

pub fn most_booked2(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable();
    let n = n as usize;
    let mut cnt = vec![0; n];
    let mut cur = vec![0; n];
    for meeting in meetings {
        let mut flag = false;
        for j in 0..n {
            if cur[j] <= meeting[0] as i64 {
                flag = true;
                cnt[j] += 1;
                cur[j] = meeting[1] as i64;
                break;
            }
        }
        if flag { continue; }
        let mut k = 0;
        for j in 1..n {
            if cur[j] < cur[k] { k = j; }
        }
        cnt[k] += 1;
        cur[k] += (meeting[1] - meeting[0]) as i64;
    }
    let mut result = 0;
    for i in 1..n {
        if cnt[i] > cnt[result] { result = i; }
    }
    result as i32
}


fn main() {
    fn test(func: fn(n: i32, meetings: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]), 0);
        assert_eq!(func(3, vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]), 1);
    }
    test(most_booked);
    test(most_booked2);
}
