// use std::collections::LinkedList;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn create_list_node(v: &Vec<i32>) -> Option<Box<ListNode>> {
    if v.is_empty() {
        return None
    }
    let n = v.len();
    let mut list_node = Box::new(ListNode::new(v[0]));
    let mut ln = &mut list_node;
    let mut index = 1;
    while index < n {
        ln.next = Some(Box::new(ListNode::new(v[index])));
        ln = ln.next.as_mut().unwrap();
        index += 1;
        // let mut node = Box::new(ListNode::new(v[index]));
        // node.next = Some(list_node);
        // list_node = node;
    }
    Some(list_node)
}

fn main() {
    // println!("{:?}", create_list_node(&vec![1, 2, 3, 4, 5]));
    println!("{:?}", remove_nth_from_end(create_list_node(&vec![1]), 1));
}

fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut d: * const _ = &head;
    for _ in 0..n - 1 {
        // d = &unsafe{ &*d }.unwrap().next;
        d = match unsafe { &*d } {
            Some(a) => &a.next,
            None => &None
        };
        // let a = unsafe { &*d }.unwrap();
        // d = &a.next;
        // if let Some(a) = unsafe { &*d } {
        //     d = &a.next;
        // }
    }

    let mut h = &mut head;
    let mut prev: &mut Option<Box<ListNode>> = &mut None;
    while let Some(a) = unsafe { &*d } {
        d = &a.next;
        prev = unsafe { &mut *h };
        h = &mut h.as_mut().unwrap().next;
    }
    // println!("{:?}", h);
    if let Some(p) = prev.as_mut() {
        p.next = h.as_mut().unwrap().next.take();
    }
    // prev.as_mut().unwrap().next = node;
    // println!("{:?}", node);
    head
    // *prev.as_mut()
}

/*fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut l: *const _ = &head;
    let mut d = head.as_mut().unwrap();
    let mut i = 0;
    while let Some(sl) = unsafe { &*l } {
        l = &sl.next;
        if i > n { d = d.next.as_mut().unwrap() }
        i += 1;
    }
    if i == n { return head.unwrap().next }
    d.next = d.next.take().unwrap().next;
    head
}*/
