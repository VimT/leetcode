//! 点菜展示表

use std::collections::{BTreeSet, HashMap, HashSet};

/// btreeset, 52ms
pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut tables = BTreeSet::new();
    let mut foods = BTreeSet::new();
    let mut cnt: HashMap<(i32, &str), i32> = HashMap::new();
    for order in &orders {
        let table: i32 = order[1].parse().unwrap();
        tables.insert(table);
        let food = &order[2];
        *cnt.entry((table, food.as_str())).or_default() += 1;
        foods.insert(food);
    }
    let mut result = Vec::with_capacity(tables.len() + 1);
    let mut head = Vec::with_capacity(foods.len() + 1);
    head.push(String::from("Table"));
    for food in &foods {
        head.push(food.to_string());
    }
    result.push(head);
    for table in tables {
        let mut line = Vec::with_capacity(foods.len() + 1);
        line.push(table.to_string());
        for food in &foods {
            line.push(cnt.get(&(table, food)).cloned().unwrap_or(0).to_string());
        }
        result.push(line);
    }
    result
}

/// hashmap + hashset, 40ms
pub fn display_table2(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut table_food: HashMap<i32, HashMap<&str, i32>> = HashMap::new();
    let mut foods: HashSet<&str> = HashSet::new();
    for order in &orders {
        let table: i32 = order[1].parse().unwrap();
        let food = &order[2];
        foods.insert(food);
        *table_food.entry(table).or_default().entry(food).or_default() += 1;
    }
    let mut foods: Vec<&str> = foods.into_iter().collect();
    foods.sort_unstable();
    let mut result = Vec::with_capacity(table_food.len() + 1);
    let mut head = Vec::with_capacity(foods.len() + 1);
    head.push(String::from("Table"));
    for food in &foods {
        head.push(food.to_string());
    }
    result.push(head);
    let mut tf : Vec<_> = table_food.into_iter().collect();
    tf.sort_unstable_by_key(|x| x.0);
    for (table, food_cnt) in tf {
        let mut line = Vec::with_capacity(foods.len() + 1);
        line.push(table.to_string());
        for food in &foods {
            line.push(food_cnt.get(food).map(|x| x.to_string()).unwrap_or(String::from("0")));
        }
        result.push(line);
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(orders: Vec<Vec<String>>) -> Vec<Vec<String>>) {
        assert_eq!(func(vec![svec!["David","3","Ceviche"], svec!["Corina","10","Beef Burrito"], svec!["David","3","Fried Chicken"], svec!["Carla","5","Water"], svec!["Carla","5","Ceviche"], svec!["Rous","3","Ceviche"]]), vec![svec!["Table","Beef Burrito","Ceviche","Fried Chicken","Water"], svec!["3","0","2","1","0"], svec!["5","0","1","0","1"], svec!["10","1","0","0","0"]]);
        assert_eq!(func(vec![svec!["James","12","Fried Chicken"], svec!["Ratesh","12","Fried Chicken"], svec!["Amadeus","12","Fried Chicken"], svec!["Adam","1","Canadian Waffles"], svec!["Brianna","1","Canadian Waffles"]]), vec![svec!["Table","Canadian Waffles","Fried Chicken"], svec!["1","2","0"], svec!["12","0","3"]]);
        assert_eq!(func(vec![svec!["Laura","2","Bean Burrito"], svec!["Jhon","2","Beef Burrito"], svec!["Melissa","2","Soda"]]), vec![svec!["Table","Bean Burrito","Beef Burrito","Soda"], svec!["2","1","1","1"]]);
    }
    test(display_table);
    test(display_table2);
}
