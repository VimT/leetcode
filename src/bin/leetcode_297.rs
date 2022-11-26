//! 二叉树的序列化与反序列化

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

struct Codec {}

impl Codec {
    fn new() -> Self { Codec {} }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Option<i32>>) {
            match root {
                None => {
                    result.push(None);
                }
                Some(v) => {
                    result.push(Some(v.borrow().val));
                    dfs(v.borrow().left.clone(), result);
                    dfs(v.borrow().right.clone(), result);
                }
            }
        }
        let mut result = vec![];
        dfs(root, &mut result);
        let s: Vec<String> = result.into_iter().map(|x| x.map(|v| v.to_string()).unwrap_or(String::from("n"))).collect();
        s.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let s: Vec<Option<i32>> = data.split(",").map(|x| match x {
            "n" => None,
            v => Some(v.parse::<i32>().unwrap()),
        }).collect();
        fn dfs(s: &Vec<Option<i32>>, idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            match s[*idx] {
                None => None,
                Some(v) => {
                    let mut node = TreeNode::new(v);
                    *idx += 1;
                    node.left = dfs(s, idx);
                    *idx += 1;
                    node.right = dfs(s, idx);
                    Some(Rc::new(RefCell::new(node)))
                }
            }
        }
        dfs(&s, &mut 0)
    }
}

fn main() {
    let c = Codec::new();
    let help = |t: Option<Rc<RefCell<TreeNode>>>| {
        assert_eq!(c.deserialize(c.serialize(t.clone())), t);
    };
    help(tree![1,2,3,null,null,4,5]);
    help(tree![]);
    help(tree![1]);
    help(tree![1,2]);
}
