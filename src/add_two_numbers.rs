use helpers::ListNode;

/**
# Problem Statement

You are given two singly-linked lists, each representing the digits of a non-negative integer in reverse order.
Compute the sum of these numbers and return it in the same format.

# Example

Problem: 342 + 465 = 807
Input: 2 -> 4 -> 3 -> None
       5 -> 6 -> 4 -> None
Output: 7 -> 0 -> 8 -> None

# Solution

The numbers are already in reverse order, so conceptually it's very simple (just add the digits and keep hold of the carry).
We will construct the resulting list from its head (i.e. least significant digit), and keep hold of a mutable pointer to the tail to append new digits.

Complexity is O(n), where n is the length of the longer list.

## Example

1. Initially, result is None and tail is a mutable reference to result.
2. First digit: 2 + 5 = 7, carry = 0:
    - Write `ListNode(7)` to the tail
    - Advance the tail pointer. (It will now point to the `next` field of the node we just created)
3. Second digit: 0 + 4 + 6 = 10, carry = 1:
    - Write `ListNode(0)` to the tail
    - Advance the tail pointer. (Now it effectively points to `result.next.next`)
4. Third digit: 1 + 4 + 3 = 8, carry = 0:
    - Append `ListNode(8)` to the tail. (You know the drill)
5. There are no more digits, and carry is 0, so we are done.
*/
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut a = l1;
    let mut b = l2;
    let mut carry = 0;
    let mut result: Option<Box<ListNode>> = None; // Last digit of the result
    let mut tail = &mut result; // Pointer to current digit of the result

    while a.is_some() || b.is_some() || carry > 0 {
        let mut sum = carry;
        if let Some(node) = a {
            sum += node.val;
            a = node.next;
        }
        if let Some(node) = b {
            sum += node.val;
            b = node.next;
        }
        carry = sum / 10;

        let new_node = Some(Box::new(ListNode::new(sum % 10)));
        *tail = new_node; // Append new digit to the list

        if let Some(t) = tail {
            // Advance the tail pointer
            tail = &mut t.next;
        }
    }

    result
}

pub mod helpers {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    pub fn parse(st: &str) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut curr: &mut Option<Box<ListNode>> = &mut head;
        for c in st.chars().rev() {
            match c.to_digit(10) {
                Some(d) => {
                    *curr = Some(Box::new(ListNode::new(d as i32)));
                    curr = &mut curr.as_mut().unwrap().next;
                }
                None => {
                    return None;
                }
            }
        }
        head
    }

    pub fn to_vec(l: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut curr = l;
        while let Some(node) = curr {
            result.push(node.val);
            curr = &node.next;
        }
        result
    }
}
