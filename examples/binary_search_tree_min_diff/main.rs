use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

type ORTree = Option<Rc<RefCell<TreeNode>>>;

fn create_tree_node(v: &Vec<i32>) -> ORTree {
    if v.is_empty() {
        return None
    }
    let root = Rc::new(RefCell::new(TreeNode::new(v[0])));
    let mut node = vec![root.clone()];
    let mut i = 1usize;
    let len = v.len();
    while !node.is_empty() {
        if i >= len {
            break;
        }
        if v[i] != -1 {
            let left = Rc::new(RefCell::new(TreeNode::new(v[i])));
            node[0].borrow_mut().left = Some(left.clone());
            node.push(left);
        } else {
            node[0].borrow_mut().left = None;
        }
        i += 1;
        if i >= len {
            break;
        }
        if v[i] != -1 {
            let right = Rc::new(RefCell::new(TreeNode::new(v[i])));
            node[0].borrow_mut().right = Some(right.clone());
            node.push(right);
        } else {
            node[0].borrow_mut().right = None;
        }
        i += 1;
        node.remove(0);
    }
    Some(root)
}

fn main() {
    let tree = create_tree_node(&vec![1,-1,3,2]);
    println!("{}", get_minimum_difference(tree));
}

fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut i = (i32::MAX, i32::MAX);
    fn mid_order(node: ORTree, i: &mut (i32, i32)) {
        if let Some(node) = node {
            mid_order(node.borrow_mut().right.take(), i);
            let (current, min) = i;
            let d = (node.borrow().val - *current).abs();
            if d < *min {
                *min = d;
            }
            *current = node.borrow().val;
            mid_order(node.borrow_mut().left.take(), i);
        }
    }
    mid_order(root, &mut i);
    i.1
}
