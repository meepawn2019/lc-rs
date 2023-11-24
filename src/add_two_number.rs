use crate::Solution;

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
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut remainder = 0;
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        let mut l1_node = &l1;
        let mut l2_node = &l2;

        while l1_node.is_some() || l2_node.is_some() {
            let l1_node_val = l1_node.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
            let l2_node_val = l2_node.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
            let added = l1_node_val + l2_node_val + remainder;
            remainder = 0;

            if added >= 10 {
                remainder = 1;
            }

            let digit = added % 10;
            let new_node = Box::new(ListNode::new(digit));
            tail.replace(new_node);
            tail = &mut tail.as_mut().unwrap().next;

            match l1_node {
                Some(node) => l1_node = &node.next,
                None => l1_node = &None,
            }
            match l2_node {
                Some(node) => l2_node = &node.next,
                None => l2_node = &None,
            }
        }

        if remainder > 0 {
            let new_node = Box::new(ListNode::new(remainder));
            tail.replace(new_node);
        }

        head
    }
}
