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

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut ans = 0;
    let mut curr = &head;
    while let Some(node) = curr {
        ans = (ans << 1) | node.val;
        curr = &node.next;
    }
    ans
}

fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut curr = &mut head;

    for &value in values.iter() {
        let new_node = Some(Box::new(ListNode::new(value)));
        if let Some(ref mut node) = curr {
            node.next = new_node;
            curr = &mut node.next;
        } else {
            *curr = new_node;
        }
    }

    head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_decimal_value() {
        assert_eq!(get_decimal_value(create_linked_list(vec![1, 0, 1])), 5);
        assert_eq!(get_decimal_value(create_linked_list(vec![0])), 0);
    }
}
