// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let num_set: std::collections::HashSet<i32> = nums.into_iter().collect();
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(mut node) = head {
            head = node.next.take();
            if !num_set.contains(&node.val) {
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

pub struct Solution;