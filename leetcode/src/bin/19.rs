// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
    fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &val in vals.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut arr = vec![];
        let mut current = head;
        while let Some(node) = current {
            arr.push(node.val);
            current = node.next;
        }
        arr.remove(arr.len() - n as usize);
        Self::create_list(arr)
    }
}

struct Solution;

#[derive(Debug, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn main() {
    let head = Solution::create_list(vec![1, 2, 3, 4, 5]);
    let n = 2;
    let res = Solution::create_list(vec![1, 2, 3, 5]); // Expected result after removing 4
    assert_eq!(Solution::remove_nth_from_end(head, n), res);
}
