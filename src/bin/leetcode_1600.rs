//! 王位继承顺序

use std::collections::HashMap;

#[derive(Default)]
struct People {
    children: Vec<String>,
    depth: bool,
}

struct ThroneInheritance {
    king_name: String,
    m: HashMap<String, People>,
}


impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        let mut m = HashMap::new();
        m.insert(king_name.clone(), People::default());
        Self { king_name, m }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        if let Some(p) = self.m.get_mut(&parent_name) {
            p.children.push(child_name.clone());
        }
        self.m.insert(child_name, People::default());
    }

    fn death(&mut self, name: String) {
        if let Some(p) = self.m.get_mut(&name) {
            p.depth = true;
        }
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        fn dfs(m: &HashMap<String, People>, cur: &People, result: &mut Vec<String>) {
            for child in &cur.children {
                let p = &m[child];
                if !p.depth { result.push(child.clone()); }
                dfs(m, p, result);
            }
        }
        let mut result = vec![];
        let p = &self.m[&self.king_name];
        if !p.depth { result.push(self.king_name.clone()); }
        dfs(&self.m, p, &mut result);
        result
    }
}

fn main() {
    let mut t = ThroneInheritance::new(String::from("king")); // 继承顺序：king
    t.birth(String::from("king"), String::from("andy")); // 继承顺序：king > andy
    t.birth(String::from("king"), String::from("bob")); // 继承顺序：king > andy > bob
    t.birth(String::from("king"), String::from("catherine")); // 继承顺序：king > andy > bob > catherine
    t.birth(String::from("andy"), String::from("matthew")); // 继承顺序：king > andy > matthew > bob > catherine
    t.birth(String::from("bob"), String::from("alex")); // 继承顺序：king > andy > matthew > bob > alex > catherine
    t.birth(String::from("bob"), String::from("asha")); // 继承顺序：king > andy > matthew > bob > alex > asha > catherine
    assert_eq!(t.get_inheritance_order(), ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]); // 返回 ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]
    t.death(String::from("bob")); // 继承顺序：king > andy > matthew > bob（已经去世）> alex > asha > catherine
    assert_eq!(t.get_inheritance_order(), ["king", "andy", "matthew", "alex", "asha", "catherine"]); // 返回 ["king", "andy", "matthew", "alex", "asha", "catherine"]
}
