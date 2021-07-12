use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

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

fn create_tree_node(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Rc::new(RefCell::new(TreeNode::new(v[0])));
    let mut node = vec![root.clone()];
    let mut i = 1usize;
    let len = v.len();
    while !node.is_empty() {
        if v[i] != 0 {
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
        if v[i] != 0 {
            let right = Rc::new(RefCell::new(TreeNode::new(v[i])));
            node[0].borrow_mut().right = Some(right.clone());
            node.push(right);
        } else {
            node[0].borrow_mut().right = None;
        }
        i += 1;
        if i >= len {
            break;
        }
        node.remove(0);
    }
    Some(root)
}

fn main() {
    let instant = Instant::now();
    let t1 = create_tree_node(&vec![10, 3, 10, 1, 6, 0, 14, 0, 0, 4, 7, 14]);// Some(Rc::new(RefCell::new(TreeNode::new())));
    println!("{:?}", find_mode(t1));
    println!("{:?}", instant.elapsed());
}

fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut record = (0, 1, 1);
    fn mid_order(node: ORTree, arr: &mut Vec<i32>, rd: &mut (i32, i32, i32)) {
        if node.is_none() {
            return
        }
        let node = node.unwrap();
        mid_order(node.borrow_mut().right.take(), arr, rd);
        let (current, same, total_same) = rd;
        let val = node.borrow().val;
        if val != *current {
            *same = 1;
            *current = val;
        } else {
            *same += 1;
        }
        if *same == *total_same {
            arr.push(val);
        } else if *same > *total_same {
            *total_same = *same;
            arr.clear();
            arr.push(val);
        }
        mid_order(node.borrow_mut().left.take(), arr, rd);
    }
    mid_order(root, &mut v, &mut record);
    v
}
