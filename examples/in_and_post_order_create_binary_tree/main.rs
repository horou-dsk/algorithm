use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;
use std::collections::HashMap;
use std::iter::Rev;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

type ORTree = Option<Rc<RefCell<TreeNode>>>;

// fn create_tree(v: &Vec<i32>) -> ORTree {
//
// }

fn create_tree_node(v: &Vec<i32>) -> ORTree {
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
    fn mid_order(node: ORTree, arr: &mut Vec<i32>) {
        if node.is_none() {
            return
        }
        let node = node.unwrap();
        mid_order(node.borrow_mut().left.take(), arr);
        let val = node.borrow().val;
        arr.push(val);
        mid_order(node.borrow_mut().right.take(), arr);
    }
    let mut v = vec![];
    // mid_order(create_tree_node(&vec![3, 9, 20, 2, -1, 15, 7]), &mut v);
    mid_order(create_tree_node(&vec![9, 2, 20, -1, -1, 15, 7, 3]), &mut v);
    println!("{:?}", v);
}

fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let d = &inorder.iter().enumerate().map(|(i, &j)| (j, i)).collect::<HashMap<i32, usize>>();
    let p = &mut postorder.into_iter().rev();
    fn f(i: usize, j: usize, d: &HashMap<i32, usize>, p: &mut Rev<IntoIter<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if i < j {
            let val = p.next().unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                right: f(d[&val] + 1, j, d, p),
                left: f(i, d[&val], d, p)
            })))
        } else {None}
    }
    return f(0, inorder.len(), d, p)
}
