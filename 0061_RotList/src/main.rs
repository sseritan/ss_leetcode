// Definition for singly-linked list.
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

struct Solution;
impl Solution {
    // "Easy" solution is cyclic list + resplit, but ownership makes this trickier in Rust
    // I'll "cheat" and unpack into a Vec
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Base case: 0 or 1 nodes
        if head.is_none() {
            return head
        }
        if head.as_ref().unwrap().next.is_none() {
            return head
        }
    
        // Unpack list (O(N) time and space)
        let mut curr = &head;
        let mut entries = vec![];
        while curr.is_some() {
            entries.push(curr.as_ref().unwrap().val);
            curr = &(curr.as_ref().unwrap().next);
        }

        // Find first entry in rotated list
        let size = entries.len() as i32;
        let cut = size - (k % size);
        let mut rotated = entries.split_off(cut as usize);
        rotated.append(&mut entries);

        // Put back into linked list
        let mut new_head: Option<Box<ListNode>> = None;
        for n in rotated.iter().rev() {
            let mut node = ListNode::new(*n);
            node.next = new_head;
            new_head = Some(Box::new(node));
        }
        new_head
    }
}

fn test(nums: Vec<i32>, k: i32) {
    println!("Testing {:?}, rotating {:?} to the right... ", nums, k);

    let mut head: Option<Box<ListNode>> = None;
    for n in nums.iter().rev() {
        let mut node = ListNode::new(*n);
        node.next = head;
        head = Some(Box::new(node));
    }

    let rhead = Solution::rotate_right(head, k);

    print!("Got: ");
    let mut curr = &rhead;
    loop {
        match curr {
            Some(node) => {
                print!(" {}", node.val);
                curr = &node.as_ref().next;
            },
            None => break,
        }
    }
    println!();
}

fn main() {
    test(vec![1, 2, 3, 4, 5], 2);
    test(vec![0, 1, 2], 4);
}
