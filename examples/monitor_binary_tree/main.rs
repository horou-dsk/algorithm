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
            right: None,
        }
    }
}

type ORTree = Option<Rc<RefCell<TreeNode>>>;

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
    let instant = Instant::now();
    // [0,null,0,null,0,null,0] 2
    // [0,0,null,0,null,0,null,null,0] 2
    // [0,0,0,null,0] 2
    // [0] 1
    // [0,0,null,0,0]
    // println!("{:#?}", t1);
    // [0,0,0,null,0,0,null,null,0] 2
    // [0,0,null,0,0,null,null,0,0,0] 3
    assert_eq!(2, min_camera_cover(create_tree_node(&vec![0, -1, 0, -1, 0, -1, 0])));
    assert_eq!(2, min_camera_cover(create_tree_node(&vec![0, 0, -1, 0, -1, 0, -1, -1, 0])));
    assert_eq!(2, min_camera_cover(create_tree_node(&vec![0, 0, 0, -1, 0])));
    assert_eq!(1, min_camera_cover(create_tree_node(&vec![0])));
    assert_eq!(1, min_camera_cover(create_tree_node(&vec![0, 0, -1, 0, 0])));
    assert_eq!(2, min_camera_cover(create_tree_node(&vec![0, 0, 0, -1, 0, 0, -1, -1, 0])));
    assert_eq!(3, min_camera_cover(create_tree_node(&vec![0, 0, -1, 0, 0, -1, -1, 0, 0, 0])));
    // println!("{:?}", min_camera_cover(create_tree_node(&vec![0,-1,0,-1,0,-1,0])));
    println!("{:?}", instant.elapsed());
}

fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let arr = mins(root);
    println!("{:?}", arr);
    arr[1]
}

fn mins(root: Option<Rc<RefCell<TreeNode>>>) -> [i32; 3] {//[假设此节点安装摄像头最小需要几个,此节点不假设安装摄像头但此节点需要被子节点监视时最少需要几个,假设父节点已安装摄像头最少需要几个]
    if let Some(r) = root {
        let mut r = r.borrow_mut();
        let l = mins(r.left.take());
        let r = mins(r.right.take());
        let i = l[2] + r[2] + 1;
        let n = i.min(l[0] + r[1]).min(l[1] + r[0]); //i.min不可省略，因为有时候安装一个节点收益更高
        [i, n, n.min(l[1] + r[1])] //n.min不可省略，因为有时候安装一个节点收益更高，而n.min带着i.min的信息
    } else { [1, 0, 0] }
}

/*fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut num = 0;
    fn post_order(node: ORTree, num: &mut i32) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        let left = node.borrow_mut().left.clone();
        let right = node.borrow_mut().right.clone();
        post_order(left.clone(), num);
        post_order(right.clone(), num);
        if let Some(left) = left {
            let mut left = left.borrow_mut();
            if left.val == 0 {
                left.val = 2;
                if let Some(right) = right {
                    right.borrow_mut().val = 2;
                }
                node.borrow_mut().val = 1;
                *num += 1;
                return;
            } else if left.val == 1 {
                node.borrow_mut().val = 2;
            }
        }
        if let Some(right) = right {
            let mut right = right.borrow_mut();
            if right.val == 0 {
                right.val = 2;
                node.borrow_mut().val = 1;
                *num += 1;
            } else if right.val == 1 {
                node.borrow_mut().val = 2;
            }
        }
    }
    post_order(root.clone(), &mut num);
    // println!("{:#?}", root);
    if let Some(node) = root {
        if node.borrow().val == 0 {
            num += 1;
        } else {
            if let Some(left) = node.borrow().left.clone() {
                if left.borrow().val == 0 {
                    num += 1;
                }
            }
            if let Some(right) = node.borrow().right.clone() {
                if right.borrow().val == 0 {
                    num += 1;
                }
            }
        }
    }
    if num == 0 {1} else {num}
}*/
