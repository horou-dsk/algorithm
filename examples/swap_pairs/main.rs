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

fn main() {
    let mut head = Box::new(ListNode::new(1));
    let mut two = Box::new(ListNode::new(2));
    let mut three = Box::new(ListNode::new(3));
    three.next = Some(Box::new(ListNode::new(4)));
    two.next = Some(three);
    head.next = Some(two);
    println!("{:?}", swap_pairs(Some(head)));
}

/*fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut r = head;
    let mut result = ListNode::new(0);
    let mut tail = &mut result;
    while let Some(mut dump) = r.take() {
        r = dump.next.take();
        if let Some(mut d) = r {
            r = d.next.take();
            d.next = Some(dump);
            tail.next = Some(d);
            tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            tail.next = Some(dump);
        }
        // let mut b = dump.take();
        // if let Some(mut d) = b {
        //     r.next = d.next.take();
        //     d.next = Some(r);
        //     return Some(d)
        // }
        // return Some(r)
    }
    result.next
}*/

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.and_then(|mut n| {
        match n.next {
            Some(mut m) => {
                n.next = swap_pairs(m.next);
                m.next = Some(n);
                Some(m)
            },
            None => Some(n)
        }
    })
}
