// 
// The task https://leetcode.com/problems/add-two-numbers/submissions/
//
// You are given two non-empty linked lists representing two non-negative integers. 
// The digits are stored in reverse order, and each of their nodes contains a single 
// digit. Add the two numbers and return the sum as a linked list.
//
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Copy, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    
    pub fn extract(l: Option<Box<ListNode>>) -> (Option<i32>, Option<Box<ListNode>>) {
        match l {
            Some(n) => {
                (Some(n.val), n.next)
            }
            None => {
                (None, None)
            }
        }
    }
    
    fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {

        
        return match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let (a, n1) = Self::extract(Some(l1));
                let (b, n2) = Self::extract(Some(l2));
                let x = a.unwrap() + b.unwrap() + carry;
                let mut node = ListNode::new(x % 10);
                node.next = Self::add(n1, n2, x / 10);
                Some(Box::new(node))           
            }
            (Some(l1), None) => {
                let (a, n1) = Self::extract(Some(l1));
                let x = a.unwrap() + carry;
                let s = if x >= 10 { x % 10} else { x };
                
                let mut node = ListNode::new(s);
                node.next = Self::add(n1, None, x / 10);
                Some(Box::new(node))
            }
             (None, Some(l2)) => {
                let (b, n2) = Self::extract(Some(l2)); 
                let x = b.unwrap() + carry;
                let s = if x >= 10 { x % 10} else { x }; 
                 
                let mut node = ListNode::new(s);
                node.next = Self::add(None, n2, x / 10);
                Some(Box::new(node))
            }
            (None, None) => {
                if carry != 0 {Some(Box::new(ListNode::new(carry)))} else {None}
            }
        };
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> (Option<Box<ListNode>>) {
            
        Self::add(l1, l2, 0)
        
   }
}
    
