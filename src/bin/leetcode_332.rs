//! 重新安排行程

use std::collections::HashMap;

/// 欧拉图  Hierholzer 算法
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for ticket in tickets {
        m.entry(ticket[0].clone()).or_default().push(ticket[1].clone());
    }
    fn dfs(m: &mut HashMap<String, Vec<String>>, cur: String, result: &mut Vec<String>) {
        unsafe {
            if m.get(&cur).is_none() { return; }
            let v = m.get_mut(&cur).unwrap() as *mut Vec<String>;
            while !(*v).is_empty() {
                let nxt = (*v).pop().unwrap();
                dfs(m, nxt.clone(), result);
                result.push(nxt);
            }
        }
    }
    for (_, v) in m.iter_mut() {
        v.sort_unstable();
        v.reverse();
    }
    let mut result = vec![];
    dfs(&mut m, String::from("JFK"), &mut result);
    result.push(String::from("JFK"));
    result.reverse();
    result
}

fn main() {
    assert_eq!(find_itinerary(vec![vec!["MUC".to_string(), "LHR".to_string()], vec!["JFK".to_string(), "MUC".to_string()], vec!["SFO".to_string(), "SJC".to_string()], vec!["LHR".to_string(), "SFO".to_string()]]), vec!["JFK".to_string(), "MUC".to_string(), "LHR".to_string(), "SFO".to_string(), "SJC".to_string()]);
    assert_eq!(find_itinerary(vec![vec!["JFK".to_string(), "SFO".to_string()], vec!["JFK".to_string(), "ATL".to_string()], vec!["SFO".to_string(), "ATL".to_string()], vec!["ATL".to_string(), "JFK".to_string()], vec!["ATL".to_string(), "SFO".to_string()]]), vec!["JFK".to_string(), "ATL".to_string(), "JFK".to_string(), "SFO".to_string(), "ATL".to_string(), "SFO".to_string()]);
}