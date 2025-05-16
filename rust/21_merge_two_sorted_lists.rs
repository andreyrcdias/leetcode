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

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, l2) => l2,
        (l1, None) => l1,
        (Some(node1), Some(node2)) => {
            if node1.val < node2.val {
                let next = merge_two_lists(node1.next, Some(node2));
                Some(Box::new(ListNode {
                    val: node1.val,
                    next,
                }))
            } else {
                let next = merge_two_lists(Some(node1), node2.next);
                Some(Box::new(ListNode {
                    val: node2.val,
                    next,
                }))
            }
        }
    }
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
    fn test_merge_two_lists() {
        assert_eq!(
            merge_two_lists(
                create_linked_list(vec![1, 2, 4]),
                create_linked_list(vec![1, 3, 4])
            ),
            create_linked_list(vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            merge_two_lists(create_linked_list(vec![]), create_linked_list(vec![])),
            create_linked_list(vec![])
        );
        assert_eq!(
            merge_two_lists(create_linked_list(vec![]), create_linked_list(vec![0])),
            create_linked_list(vec![0])
        );
    }
}
