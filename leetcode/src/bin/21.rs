#![allow(dead_code)]

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        while list1.is_some() && list2.is_some() {
            let (val1, val2) = (list1.as_ref().unwrap().val, list2.as_ref().unwrap().val);
            if val1 < val2 {
                let next = list1.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = list2;
                list2 = next;
            }
            current = current.unwrap().next.as_mut();
        }
        if list1.is_some() {
            current.as_mut().unwrap().next = list1;
        } else {
            current.as_mut().unwrap().next = list2;
        }
        head.unwrap().next
    }
}

struct Solution;

fn main() {
    // [1,2,4]
    // [1,3,4]
    assert_eq!(
        Solution::merge_two_lists(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }))
    );
}
