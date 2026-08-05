use std::io::{self, Read};

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

// fn naive_add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let n1 = linked_list_to_u128(l1);
//     let n2 = linked_list_to_u128(l2);
//
//     u128_to_linked_list(n1 + n2)
// }

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = None;
    let mut tail = &mut result;

    let mut carry = 0;
    let (mut curr1, mut curr2) = (l1, l2);
    loop {
        let curr_computed: i32;
        match (curr1, curr2) {
            (Some(node1), Some(node2)) => {
                curr_computed = node1.val + node2.val + carry;
                curr1 = node1.next;
                curr2 = node2.next;
            }
            (Some(node1), None) => {
                curr_computed = node1.val + carry;
                curr1 = node1.next;
                curr2 = None;
            }
            (None, Some(node2)) => {
                curr_computed = node2.val + carry;
                curr1 = None;
                curr2 = node2.next;
            }
            (None, None) => {
                if carry != 0 {
                    *tail = Some(Box::new(ListNode::new(carry)));
                }
                break;
            }
        }

        carry = curr_computed / 10;
        *tail = Some(Box::new(ListNode::new((curr_computed) % 10)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    return result;
}

fn linked_list_to_u128(l: Option<Box<ListNode>>) -> u128 {
    let mut n: u128 = 0;
    let mut positional_weight: u128 = 1;
    let mut curr = l;
    while let Some(node) = curr {
        n += node.val as u128 * positional_weight;
        positional_weight *= 10;
        curr = node.next;
    }
    return n as u128;
}

fn u128_to_linked_list(mut n: u128) -> Option<Box<ListNode>> {
    let mut l: Option<Box<ListNode>> = None;
    let mut tail = &mut l;

    loop {
        let remainder = n % 10;
        *tail = Some(Box::new(ListNode::new(remainder as i32)));
        tail = &mut tail.as_mut().unwrap().next;

        n /= 10;
        if n == 0 {
            return l;
        }
    }
}

fn main() {
    let mut scan = Scanner::new();

    let l1 = u128_to_linked_list(scan.next());
    let l2 = u128_to_linked_list(scan.next());

    dbg!(linked_list_to_u128(add_two_numbers(l1, l2)));
}

struct Scanner {
    input: Vec<String>,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();

        Self {
            input: input.split_whitespace().rev().map(String::from).collect(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.input.pop().unwrap().parse().ok().unwrap()
    }
}
