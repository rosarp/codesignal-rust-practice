struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

impl<T> List<T> {
    fn new(v: T) -> Self {
        List {
            value: v,
            next: None,
        }
    }
}

type ListNode<T> = Option<Box<List<T>>>;

fn remove_k_from_list(mut list_node: ListNode<i32>, k: i32) -> ListNode<i32> {
    let mut current_node = &mut list_node;
    loop {
        match current_node {
            Some(node) if node.value == k => {
                *current_node = node.next.take();
            }
            Some(node) => current_node = &mut node.next,
            None => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_k_from_list() {
        let mut list_node = Some(Box::new(List::new(3)));
        let mut out_node = remove_k_from_list(list_node, 3);
        // assert
    }
}
