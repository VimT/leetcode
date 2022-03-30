//! 序列化和反序列化二叉搜索树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::treenode::{NodeTravel, TreeNode, vec_to_tree};

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<u8>) {
            if let Some(node) = root.as_ref() {
                result.extend_from_slice(format!("{:x}", node.borrow().val).as_bytes());
                result.push(b',');
                dfs(node.borrow().left.clone(), result);
                dfs(node.borrow().right.clone(), result);
            } else {
                result.push(b'n');
            }
        }
        let mut result = vec![];
        dfs(root, &mut result);
        unsafe { String::from_utf8_unchecked(result) }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(data: &[u8], idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            return if data[*idx] == b'n' {
                *idx += 1;
                None
            } else {
                let mut num = 0;
                while data[*idx] != b',' {
                    let val = if data[*idx] >= b'a' {
                        10 + data[*idx] - b'a'
                    } else {
                        data[*idx] - b'0'
                    };
                    num = num * 16 + val as i32;
                    *idx += 1;
                }
                let mut result = TreeNode::new(num);
                *idx += 1;
                result.left = dfs(data, idx);
                result.right = dfs(data, idx);
                Some(Rc::new(RefCell::new(result)))
            };
        }
        dfs(data.as_bytes(), &mut 0)
    }
}

fn main() {
    let t = vec![210123, 1, 0, 0, 3];
    let codec = Codec::new();
    let serialize = codec.serialize(vec_to_tree(t.clone()));
    assert_eq!(NodeTravel(codec.deserialize(serialize)).preorder(), vec![210123, 1, 3]);
}
