pub mod algorithm;
pub mod gcd;
pub mod linknode;
pub mod treenode;
pub mod suffix_array;
pub mod nested_integer;
pub mod union_find;
pub mod graph;
pub mod segment_tree;
pub mod bit_index_tree;
pub mod treap;
pub mod multi_set;
pub mod bitset;

pub fn unorder<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort_unstable();
    list
}

pub fn unorder_deep<T: Ord>(mut list: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for item in &mut list {
        item.sort_unstable();
    }
    list.sort_unstable();
    list
}

#[macro_export]
macro_rules! svec {
    () => (
        Vec::<String>::new()
    );
    ($($x:expr),+ $(,)?) => (
        vec![$(String::from($x)),+]
    );
}

#[macro_export]
macro_rules! link {
    () => (
        Option::<Box<ListNode>>::None
    );
    ($($x:expr),+ $(,)?) => (
        $crate::linknode::vec_to_link(vec![$($x),+])
    );
}

#[macro_export]
macro_rules! tree {
    () => (
        Option::<Rc<RefCell<TreeNode>>>::None
    );
    ($($x:expr),+$(,)?) => (
        $crate::treenode::leetcode_tree(stringify!($($x),+))
    );
}

#[macro_export]
macro_rules! nested_int {
    ($($x:expr),+$(,)?) => (
        NestedInteger::from_str(stringify!($($x),+))
    );
}
