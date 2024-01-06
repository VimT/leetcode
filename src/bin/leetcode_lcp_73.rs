//! 探险营地

use std::collections::HashSet;

pub fn adventure_camp(expeditions: Vec<String>) -> i32 {
    let mut known = HashSet::new();
    let mut result = -1;
    let mut mx = 0;
    for cmp in expeditions[0].split("->") {
        known.insert(cmp.to_string());
    }
    for (i, exp) in expeditions.into_iter().enumerate().skip(1) {
        let mut num = 0;
        for cmp in exp.split("->") {
            if !cmp.is_empty() && known.insert(cmp.to_string()) {
                num += 1;
            }
        }
        if num > mx {
            mx = num;
            result = i as i32;
        }
    }
    result
}


fn main() {
    use leetcode::svec;
    fn test(func: fn(expeditions: Vec<String>) -> i32) {
        assert_eq!(func(svec!["Alice->Dex","","Dex"]), -1);
        assert_eq!(func(svec!["leet->code","leet->code->Campsite->Leet","leet->code->leet->courier"]), 1);
        assert_eq!(func(svec!["","Gryffindor->Slytherin->Gryffindor","Hogwarts->Hufflepuff->Ravenclaw"]), 2);
    }
    test(adventure_camp);
}
