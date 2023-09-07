fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from(value: Vec<i32>) -> Option<Box<ListNode>> {
        let mut values = value.into_iter();
        let mut root_node = Box::new(ListNode::new(values.next()?));
        let mut current_node = root_node.as_mut();
        for val in values {
            current_node.next = Some(Box::new(ListNode::new(val)));
            current_node = current_node.next.as_mut()?.as_mut();
        }
        Some(root_node)
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut root_node = Box::new(ListNode::new(0));
        let mut carry = false;
        let mut current_node = root_node.as_mut();
        loop {
            let sum = match (l1, l2) {
                (None, None) => {
                    if carry {
                        current_node.next.as_mut().map(|node| node.val = 1);
                    } else if current_node.next.is_some() {
                      current_node.next = None;
                    }
                    break Some(root_node);
                }
                (None, Some(l2_node)) => {
                  l2 = l2_node.next;
                  l1 = None;
                  l2_node.val
                },
                (Some(l1_node), None) => {
                  l1 = l1_node.next;
                  l2 = None;
                  l1_node.val
                },
                (Some(l1_node), Some(l2_node)) => {
                  l1 = l1_node.next;
                  l2 = l2_node.next;
                  l1_node.val + l2_node.val
                },
            } + if carry { 1 } else { 0 };
            if current_node.next.is_some() {
              current_node = unsafe { current_node.next.as_mut().unwrap_unchecked().as_mut() }
            }
            if sum >= 10 {
                current_node.val = sum - 10;
                carry = true;
            } else {
                current_node.val = sum;
                carry = false;
            }
            current_node.next = Some(Box::new(ListNode::new(0)));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ListNode, Solution};

    #[test]
    fn example_01() {
        assert_eq!(
            Solution::add_two_numbers(ListNode::from(vec![2, 4, 3]), ListNode::from(vec![5, 6, 4])),
            ListNode::from(vec![7, 0, 8])
        )
    }
}
