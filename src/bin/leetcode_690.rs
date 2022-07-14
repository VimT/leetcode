//! 员工的重要性

use std::collections::HashMap;

struct Employee {
    id: i32,
    importance: i32,
    subordinates: Vec<i32>,
}

impl Employee {
    pub fn new(id: i32, importance: i32, subordinates: Vec<i32>) -> Self {
        Self { id, importance, subordinates }
    }
}

fn get_importance(employees: Vec<Employee>, id: i32) -> i32 {
    let mut map = HashMap::new();
    for employee in employees {
        map.insert(employee.id, employee);
    }
    fn dfs(map: &HashMap<i32, Employee>, cur: i32) -> i32 {
        let employee = &map[&cur];
        let mut result = employee.importance;
        for &sub in &employee.subordinates {
            result += dfs(map, sub);
        }
        result
    }
    dfs(&map, id)
}

fn main() {
    assert_eq!(get_importance(vec![Employee::new(1, 5, vec![2, 3]), Employee::new(2, 3, vec![]), Employee::new(3, 3, vec![])], 1), 11);
}