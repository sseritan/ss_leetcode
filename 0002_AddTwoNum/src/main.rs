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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1pos = &l1;
        let mut l2pos = &l2;
        let mut carry = 0;

        let mut l3vec = vec![];
        loop {
            let mut sum = carry;
            if let Some(node) = l1pos {
                sum += node.val;
            }
            if let Some(node) = l2pos {
                sum += node.val;
            }

            if sum == 0 && l1pos.is_none() && l2pos.is_none() {
                break;
            }

            l3vec.insert(0, sum % 10);

            carry = sum / 10;
            match l1pos.as_ref() {
                None => {
                    l1pos = &None;
                },
                Some(node) => {
                    l1pos = &node.next;
                }
            }
            match l2pos.as_ref() {
                None => {
                    l2pos = &None;
                },
                Some(node) => {
                    l2pos = &node.next;
                }
            }
        }
        
        arr_to_list(l3vec.as_slice())
    }
}

// Return linked list from array of ints
fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for num in arr {
        let mut node = ListNode::new(*num);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn print_list(mut l: &Option<Box<ListNode>>, label: &str) {
    print!("List {}: ", label);
    loop {
        match l {
            None => {
                println!("None");
                break;
            },
            Some(node) => {
                print!("{} -> ", node.val);
                l = &node.next;
            }
        }
    }
}

fn main() {
    // 342
    let l1 = arr_to_list(&[3, 4, 2, 6]);
    let l1 = arr_to_list(&[0]);
    print_list(&l1, "l1");

    // 465
    let l2 = arr_to_list(&[4, 6, 5, 7, 5]);
    let l2 = arr_to_list(&[0]);
    print_list(&l2, "l2");

    // Answer is 807
    let l3 = Solution::add_two_numbers(l1, l2);
    print_list(&l3, "l3");
}
