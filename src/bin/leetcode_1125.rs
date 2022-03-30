//! 最小的必要团队

use std::collections::HashMap;

use leetcode::{svec, unorder};

/// 还可以减枝
pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    let len = req_skills.len();
    let mut skill_map = HashMap::new();
    for (i, skill) in req_skills.into_iter().enumerate() {
        skill_map.insert(skill, i);
    }
    let people: Vec<i32> = people.into_iter().map(|x| {
        let mut skill_bit = 0;
        for skill in x {
            if let Some(idx) = skill_map.get(&skill) {
                skill_bit |= 1 << idx;
            }
        }
        skill_bit
    }).collect();
    let mut dp = vec![i32::MAX - 10; 1 << len];
    let mut next = vec![(0, 0); 1 << len];
    dp[0] = 0;
    for i in 1..1 << len {
        for (pi, &has_skill) in people.iter().enumerate() {
            let pre = i & !has_skill;
            if dp[pre as usize] + 1 < dp[i as usize] {
                dp[i as usize] = dp[pre as usize] + 1;
                next[i as usize] = (pi, pre);
            }
        }
    }
    let mut cur = (1 << len) - 1;
    let mut result = vec![];
    while cur > 0 {
        let (pi, next_state) = next[cur as usize];
        result.push(pi as i32);
        cur = next_state;
    }
    result
}

/// 回溯法  执行耗时差一个数量级。。
pub fn smallest_sufficient_team_dfs(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    let len = req_skills.len();
    let mut skill_map = HashMap::new();
    for (i, skill) in req_skills.into_iter().enumerate() {
        skill_map.insert(skill, i);
    }
    let people: Vec<i32> = people.into_iter().map(|x| {
        let mut skill_bit = 0;
        for skill in x {
            if let Some(idx) = skill_map.get(&skill) {
                skill_bit |= 1 << idx;
            }
        }
        skill_bit
    }).collect();
    fn dfs(people: &Vec<i32>, final_state: i32, state: i32, cur_people: &mut Vec<i32>, start_people: usize, result: &mut Vec<i32>) {
        if state == final_state {
            if result.is_empty() || cur_people.len() < result.len() {
                *result = cur_people.clone();
            }
        }
        if !result.is_empty() && cur_people.len() >= result.len() {
            return;
        }
        for i in start_people..people.len() {
            if people[i] | state != state {
                cur_people.push(i as i32);
                dfs(people, final_state, people[i] | state, cur_people, i + 1, result);
                cur_people.pop();
            }
        }
    }
    let mut result = vec![];
    dfs(&people, (1 << len) - 1, 0, &mut vec![], 0, &mut result);
    result
}

/// dp减枝
pub fn smallest_sufficient_team_dp(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    let len = req_skills.len();
    let mut skill_map = HashMap::new();
    for (i, skill) in req_skills.into_iter().enumerate() {
        skill_map.insert(skill, i);
    }
    let people: Vec<i32> = people.into_iter().map(|x| {
        let mut skill_bit = 0;
        for skill in x {
            if let Some(idx) = skill_map.get(&skill) {
                skill_bit |= 1 << idx;
            }
        }
        skill_bit
    }).collect();
    let mut dp = vec![i32::MAX - 10; 1 << len];
    let mut next_state = vec![(0, 0); 1 << len];
    dp[0] = 0;
    let mut max_state = 0;
    for (pi, &has_skill) in people.iter().enumerate() {
        if has_skill == 0 { continue; }
        for i in 0..=max_state {
            let next = i | has_skill;
            if dp[i as usize] + 1 < dp[next as usize] {
                dp[next as usize] = dp[i as usize] + 1;
                next_state[next as usize] = (pi, i);
            }
        }
        max_state = max_state | has_skill;
    }
    let mut cur = (1 << len) - 1;
    let mut result = vec![];
    while cur > 0 {
        let (pi, next_state) = next_state[cur as usize];
        result.push(pi as i32);
        cur = next_state;
    }
    result
}


fn main() {
    fn test(func: fn(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32>) {
        assert_eq!(unorder(func(svec!["java","nodejs","reactjs"], vec![svec!["java"], svec!["nodejs"], svec!["nodejs","reactjs"]])), vec![0, 2]);
        assert_eq!(unorder(func(svec!["algorithms","math","java","reactjs","csharp","aws"], vec![svec!["algorithms","math","java"], svec!["algorithms","math","reactjs"], svec!["java","csharp","aws"], svec!["reactjs","csharp"], svec!["csharp","math"], svec!["aws","java"]])), vec![1, 2]);
    }
    test(smallest_sufficient_team);
    test(smallest_sufficient_team_dfs);
    test(smallest_sufficient_team_dp);
}
